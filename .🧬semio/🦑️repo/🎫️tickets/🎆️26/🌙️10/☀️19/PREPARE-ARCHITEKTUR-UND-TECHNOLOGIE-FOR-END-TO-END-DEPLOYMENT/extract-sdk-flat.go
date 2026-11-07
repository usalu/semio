package main

import (
    "context"
    "crypto/sha256"
    "encoding/hex"
    "encoding/json"
    "fmt"
    "io"
    "os"
    "os/signal"
    "path/filepath"
    "regexp"
    "sort"
    "strings"
    "syscall"
    "unsafe"
)

type fileInfo struct { Name *uint16; Size uint32; Error uint32; Date uint16; Time uint16; Attributes uint16; Target [260]uint16 }
type file struct { Index int `json:"index"`; ID string `json:"id"`; Cabinet string `json:"cabinet"`; Size int64 `json:"size"` }
type payload struct { Name string `json:"fileName"`; Hash string `json:"sha256"`; Size int64 `json:"size"` }
type pkg struct { ID string `json:"id"`; Payloads []payload `json:"payloads"` }
type manifest struct { Packages []pkg `json:"packages"` }
func text(p *uint16) string { if p == nil { panic("missing name") }; a := make([]uint16, 0, 128); for i := uintptr(0); i < 128; i++ { c := *(*uint16)(unsafe.Add(unsafe.Pointer(p), i*2)); if c == 0 { return syscall.UTF16ToString(a) }; a = append(a, c) }; panic("name too long") }
func read(path string) []byte { b, e := os.ReadFile(path); if e != nil { panic(e) }; return b }
func digest(b []byte) string { s := sha256.Sum256(b); return hex.EncodeToString(s[:]) }
func lockFile(path string, size int64, hash string) *os.File {
    p, err := syscall.UTF16PtrFromString(path); if err != nil { panic(err) }
    h, err := syscall.CreateFile(p, syscall.GENERIC_READ, syscall.FILE_SHARE_READ, nil, syscall.OPEN_EXISTING, syscall.FILE_ATTRIBUTE_NORMAL, 0); if err != nil { panic(err) }
    f := os.NewFile(uintptr(h), path); if f == nil { panic("open payload") }; stat, err := f.Stat(); if err != nil || stat.Size() != size { panic("payload size mismatch") }
    d := sha256.New(); if _, err := io.Copy(d, f); err != nil { panic(err) }; if hex.EncodeToString(d.Sum(nil)) != strings.ToLower(hash) { panic("payload hash mismatch") }
    return f
}
func table(db uint32, query string, width int) [][]string {
    dll := syscall.NewLazyDLL("msi.dll"); var view uint32; q, _ := syscall.UTF16PtrFromString(query)
    r, _, _ := dll.NewProc("MsiDatabaseOpenViewW").Call(uintptr(db), uintptr(unsafe.Pointer(q)), uintptr(unsafe.Pointer(&view))); if r != 0 { panic("MSI view") }; defer dll.NewProc("MsiCloseHandle").Call(uintptr(view))
    r, _, _ = dll.NewProc("MsiViewExecute").Call(uintptr(view), 0); if r != 0 { panic("MSI execute") }; result := [][]string{}
    for { var record uint32; r, _, _ = dll.NewProc("MsiViewFetch").Call(uintptr(view), uintptr(unsafe.Pointer(&record))); if r == 259 { break }; if r != 0 { panic("MSI fetch") }; row := make([]string, width)
        for i := range row { n := uint32(4096); b := make([]uint16, n); r, _, _ = dll.NewProc("MsiRecordGetStringW").Call(uintptr(record), uintptr(i+1), uintptr(unsafe.Pointer(&b[0])), uintptr(unsafe.Pointer(&n))); if r != 0 { panic("MSI field") }; row[i] = syscall.UTF16ToString(b) }; dll.NewProc("MsiCloseHandle").Call(uintptr(record)); result = append(result, row) }
    return result
}
func main() {
    if len(os.Args) != 2 { panic("expected owned generated alias") }; root := os.Args[1]
    data := read(filepath.Join(root, "bazel-msvc-manifest.json")); if digest(data) != "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef" { panic("manifest hash") }
    var m manifest; if err := json.Unmarshal(data, &m); err != nil { panic(err) }
    payloads := map[string]payload{}; for _, p := range m.Packages { if p.ID == "Win11SDK_10.0.26100" { for _, x := range p.Payloads { payloads[strings.ToLower(filepath.Base(strings.ReplaceAll(x.Name, "\\", "/")))] = x } } }
    var files []file; if err := json.Unmarshal(read(filepath.Join(root, "sdk-file-plan.json")), &files); err != nil { panic(err) }
    validID := regexp.MustCompile(`^[a-zA-Z0-9_]+$`); validCAB := regexp.MustCompile(`^[a-f0-9]{32}\.cab$`)
    groups := map[string]map[string]int64{}; ids := map[string]bool{}; var total int64
    for _, f := range files { if !validID.MatchString(f.ID) || !validCAB.MatchString(f.Cabinet) || f.Size < 0 || f.Size > 100_000_000 || ids[strings.ToLower(f.ID)] { panic("invalid file plan") }; ids[strings.ToLower(f.ID)] = true; if groups[f.Cabinet] == nil { groups[f.Cabinet] = map[string]int64{} }; groups[f.Cabinet][f.ID] = f.Size; total += f.Size }
    if len(files) != 4453 || len(groups) != 34 || total != 1113686631 { panic("unexpected file plan") }
    msiNames := []string{"Windows SDK for Windows Store Apps Tools-x86_en-us.msi", "Windows SDK for Windows Store Apps Headers-x86_en-us.msi", "Windows SDK for Windows Store Apps Headers OnecoreUap-x86_en-us.msi", "Windows SDK for Windows Store Apps Libs-x86_en-us.msi", "Universal CRT Headers Libraries and Sources-x86_en-us.msi", "Windows SDK Desktop Headers x64-x86_en-us.msi", "Windows SDK OnecoreUap Headers x64-x86_en-us.msi", "Windows SDK Desktop Libs x64-x86_en-us.msi"}
    held := []*os.File{}; defer func() { for _, f := range held { f.Close() } }()
    planned := map[string]file{}; for _, f := range files { planned[fmt.Sprintf("%d/%s", f.Index, f.ID)] = f }
    for index, name := range msiNames {
        p, ok := payloads[strings.ToLower(name)]; if !ok { panic("missing MSI payload") }; path := filepath.Join(root, fmt.Sprintf("sdk-%d.msi", index)); held = append(held, lockFile(path, p.Size, p.Hash))
        var db uint32; pointer, _ := syscall.UTF16PtrFromString(path); dll := syscall.NewLazyDLL("msi.dll"); r, _, _ := dll.NewProc("MsiOpenDatabaseW").Call(uintptr(unsafe.Pointer(pointer)), 0, uintptr(unsafe.Pointer(&db))); if r != 0 { panic("MSI database") }
        media := table(db, "SELECT `DiskId`, `LastSequence`, `Cabinet` FROM `Media`", 3); sort.Slice(media, func(i,j int) bool { var a,b int; fmt.Sscan(media[i][0], &a); fmt.Sscan(media[j][0], &b); return a < b })
        for _, row := range table(db, "SELECT `File`, `FileSize`, `Sequence` FROM `File`", 3) { var size, seq int64; if _, err := fmt.Sscan(row[1], &size); err != nil { panic(err) }; if _, err := fmt.Sscan(row[2], &seq); err != nil { panic(err) }; cabinet := ""; for _, medium := range media { var last int64; if _, err := fmt.Sscan(medium[1], &last); err != nil { panic(err) }; if last >= seq { cabinet = medium[2]; break } }; key := fmt.Sprintf("%d/%s", index, row[0]); f, ok := planned[key]; if !ok || f.Size != size || f.Cabinet != cabinet { panic("MSI file plan mismatch: " + key) }; delete(planned, key) }
        dll.NewProc("MsiCloseHandle").Call(uintptr(db))
    }
    if len(planned) != 0 { panic("extra file plan entries") }
    names := make([]string, 0, len(groups)); for name := range groups { names = append(names, name) }; sort.Strings(names)
    for _, name := range names { p, ok := payloads[name]; if !ok { panic("missing cabinet payload") }; held = append(held, lockFile(filepath.Join(root, name), p.Size, p.Hash)) }
    dest, err := os.MkdirTemp(root, "sdk-flat-"); if err != nil { panic(err) }
    ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt); defer stop()
    var expected map[string]int64; seen := map[string]bool{}; var callbackError string
    callback := syscall.NewCallback(func(_ uintptr, notification uint32, first, _ uintptr) uintptr {
        if ctx.Err() != nil { callbackError = "canceled"; return 0 }
        if notification == 0x11 {
            info := (*fileInfo)(unsafe.Pointer(first)); name := text(info.Name); size, ok := expected[name]
            if !ok || seen[name] || size != int64(info.Size) { callbackError = "unexpected cabinet member " + name; return 0 }
            path := filepath.Join(dest, name); encoded, e := syscall.UTF16FromString(path)
            if e != nil || len(encoded) > len(info.Target) { callbackError = "target path too long"; return 0 }
            copy(info.Target[:], encoded); seen[name] = true; return 1
        }
        if notification == 0x12 { callbackError = "continued cabinet unsupported"; return 1 }; return 0
    })
    api := syscall.NewLazyDLL("setupapi.dll").NewProc("SetupIterateCabinetW")
    for _, name := range names {
        expected = groups[name]; cab, e := syscall.UTF16PtrFromString(filepath.Join(root, name)); if e != nil { panic(e) }
        ok, _, callErr := api.Call(uintptr(unsafe.Pointer(cab)), 0, callback, 0)
        if ok == 0 || callbackError != "" { panic(fmt.Sprintf("%s: %s %v", name, callbackError, callErr)) }
        fmt.Printf("Extracted %s\n", name)
    }
    if len(seen) != len(files) { panic("missing cabinet members") }
    for _, f := range files { info, err := os.Stat(filepath.Join(dest, f.ID)); if err != nil || !info.Mode().IsRegular() || info.Size() != f.Size { panic("extracted file size mismatch: " + f.ID) } }
    fmt.Printf("Verified %d files in %s\n", len(files), dest)
}
