package main

import (
    "encoding/json"
    "fmt"
    "os"
    "path/filepath"
    "syscall"
    "unsafe"
)

var msi = syscall.NewLazyDLL("msi.dll")
var open = msi.NewProc("MsiOpenDatabaseW")
var view = msi.NewProc("MsiDatabaseOpenViewW")
var execute = msi.NewProc("MsiViewExecute")
var fetch = msi.NewProc("MsiViewFetch")
var field = msi.NewProc("MsiRecordGetStringW")
var closeHandle = msi.NewProc("MsiCloseHandle")

type entry struct { Index int `json:"index"`; Cabinet string `json:"cabinet"` }

func check(code uintptr, op string) { if code != 0 { panic(fmt.Sprintf("%s: MSI error %d", op, code)) } }
func ptr(s string) *uint16 { p, err := syscall.UTF16PtrFromString(s); if err != nil { panic(err) }; return p }
func cabinets(path string, index int) []entry {
    var db, query uint32
    code, _, _ := open.Call(uintptr(unsafe.Pointer(ptr(path))), 0, uintptr(unsafe.Pointer(&db))); check(code, "open database")
    defer closeHandle.Call(uintptr(db))
    code, _, _ = view.Call(uintptr(db), uintptr(unsafe.Pointer(ptr("SELECT `Cabinet` FROM `Media`"))), uintptr(unsafe.Pointer(&query))); check(code, "open view")
    defer closeHandle.Call(uintptr(query))
    code, _, _ = execute.Call(uintptr(query), 0); check(code, "execute view")
    var rows []entry
    for {
        var record uint32
        code, _, _ = fetch.Call(uintptr(query), uintptr(unsafe.Pointer(&record)))
        if code == 259 { break }; check(code, "fetch")
        size := uint32(1024)
        buffer := make([]uint16, size)
        code, _, _ = field.Call(uintptr(record), 1, uintptr(unsafe.Pointer(&buffer[0])), uintptr(unsafe.Pointer(&size)))
        closeHandle.Call(uintptr(record)); check(code, "read cabinet")
        name := syscall.UTF16ToString(buffer)
        if name != "" { rows = append(rows, entry{index, name}) }
    }
    return rows
}
func main() {
    if len(os.Args) != 2 { panic("expected generated directory") }
    rows := []entry{}
    for i := 0; i < 8; i++ { rows = append(rows, cabinets(filepath.Join(os.Args[1], fmt.Sprintf("sdk-%d.msi", i)), i)...)}
    output, err := os.OpenFile(filepath.Join(os.Args[1], "sdk-cabinets.json"), os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600); if err != nil { panic(err) }
    defer output.Close()
    if err := json.NewEncoder(output).Encode(rows); err != nil { panic(err) }
    fmt.Printf("%d cabinet entries\n", len(rows))
}
