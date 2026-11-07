package main

import (
    "crypto/sha256"
    "encoding/hex"
    "encoding/json"
    "fmt"
    "io"
    "os"
    "path/filepath"
    "syscall"
    "unsafe"
)

var dll = syscall.NewLazyDLL("msi.dll")
var openDB = dll.NewProc("MsiOpenDatabaseW")
var openView = dll.NewProc("MsiDatabaseOpenViewW")
var execute = dll.NewProc("MsiViewExecute")
var fetch = dll.NewProc("MsiViewFetch")
var getString = dll.NewProc("MsiRecordGetStringW")
var closeHandle = dll.NewProc("MsiCloseHandle")

func check(code uintptr, operation string) { if code != 0 { panic(fmt.Sprintf("%s: MSI error %d", operation, code)) } }
func str(s string) *uint16 { p, err := syscall.UTF16PtrFromString(s); if err != nil { panic(err) }; return p }
func rows(db uint32, query string, columns int) [][]string {
    var view uint32
    code, _, _ := openView.Call(uintptr(db), uintptr(unsafe.Pointer(str(query))), uintptr(unsafe.Pointer(&view))); check(code, "open view")
    defer closeHandle.Call(uintptr(view))
    code, _, _ = execute.Call(uintptr(view), 0); check(code, "execute view")
    result := [][]string{}
    for {
        var record uint32
        code, _, _ = fetch.Call(uintptr(view), uintptr(unsafe.Pointer(&record)))
        if code == 259 { break }; check(code, "fetch")
        row := make([]string, columns)
        for i := range row {
            size := uint32(4096); buffer := make([]uint16, size)
            code, _, _ = getString.Call(uintptr(record), uintptr(i+1), uintptr(unsafe.Pointer(&buffer[0])), uintptr(unsafe.Pointer(&size))); check(code, "read field")
            row[i] = syscall.UTF16ToString(buffer)
        }
        closeHandle.Call(uintptr(record)); result = append(result, row)
    }
    return result
}
func main() {
    if len(os.Args) != 3 { panic("expected generated directory and output prefix") }
    hashes := []string{"8fb00df405939d60e50a2f1f5d068284153ab3520b6b3657c9ce3796e4a60469", "c9f78ae3e730ea1789a37001e204a970099d2acc555cc537fb45ea0bb23b347c", "be5ee13ccc98b87211b4f3686972b00fa16f20f266fe9dc5572d62f001e0547c", "1381535d2f6b1894a092da67f8a8fb048a4dfe8060ce75b3275affa81b02586e", "f611ce8a9e576e3383917b04b6fbe5ee6bed8363c1a2a8e9d6f8335cbb422675", "d189ca50e5632b546795922e2262794c068d6ff301860feb4522b8b93cbb3ba8", "14667a930569186a25e75eb35614ce43549b7406705e098db71ffe9b31c60faa", "2e956ca1cf17800b0f9811a6249b945f045addf18ee85ffe7aea99ec6c27243a"}
    if os.Args[2] != "sdk-recheck" { panic("unsupported output prefix") }
    for i := 0; i < 8; i++ {
        var db uint32
        path := filepath.Join(os.Args[1], fmt.Sprintf("sdk-%d.msi", i))
        handle, err := syscall.CreateFile(str(path), syscall.GENERIC_READ, syscall.FILE_SHARE_READ, nil, syscall.OPEN_EXISTING, syscall.FILE_ATTRIBUTE_NORMAL, 0); if err != nil { panic(err) }; locked := os.NewFile(uintptr(handle), path); if locked == nil { panic("lock MSI") }; digest := sha256.New(); if _, err := io.Copy(digest, locked); err != nil { panic(err) }; if hex.EncodeToString(digest.Sum(nil)) != hashes[i] { panic("MSI digest mismatch") }
        code, _, _ := openDB.Call(uintptr(unsafe.Pointer(str(path))), 0, uintptr(unsafe.Pointer(&db))); check(code, "open database")
        result := map[string][][]string{
            "files": rows(db, "SELECT `File`, `Component_`, `FileName`, `FileSize`, `Sequence` FROM `File`", 5),
            "components": rows(db, "SELECT `Component`, `Directory_` FROM `Component`", 2),
            "directories": rows(db, "SELECT `Directory`, `Directory_Parent`, `DefaultDir` FROM `Directory`", 3),
            "media": rows(db, "SELECT `DiskId`, `LastSequence`, `Cabinet` FROM `Media`", 3),
        }
        closeHandle.Call(uintptr(db))
        if err := locked.Close(); err != nil { panic(err) }
        out, err := os.OpenFile(filepath.Join(os.Args[1], fmt.Sprintf("%s-%d.json", os.Args[2], i)), os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600); if err != nil { panic(err) }
        if err := json.NewEncoder(out).Encode(result); err != nil { panic(err) }; if err := out.Close(); err != nil { panic(err) }
        fmt.Printf("MSI %d: %d files, %d directories\n", i, len(result["files"]), len(result["directories"]))
    }
}
