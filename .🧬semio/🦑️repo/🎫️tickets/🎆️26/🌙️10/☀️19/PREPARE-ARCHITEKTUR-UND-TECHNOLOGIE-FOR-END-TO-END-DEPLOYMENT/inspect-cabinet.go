package main

import (
    "encoding/json"
    "fmt"
    "os"
    "syscall"
    "unsafe"
)

type fileInfo struct {
    Name *uint16
    Size uint32
    Error uint32
    Date uint16
    Time uint16
    Attributes uint16
    Target [260]uint16
}
type entry struct { Name string `json:"name"`; Size uint32 `json:"size"` }

func text(p *uint16) string {
    if p == nil { panic("missing cabinet member name") }
    data := make([]uint16, 0, 256)
    for i := uintptr(0); i < 1024; i++ {
        c := *(*uint16)(unsafe.Add(unsafe.Pointer(p), i*2))
        if c == 0 { return syscall.UTF16ToString(data) }
        data = append(data, c)
    }
    panic("cabinet member name exceeds bound")
}
func main() {
    if len(os.Args) != 3 { panic("expected cabinet and output paths") }
    files := []entry{}
    callback := syscall.NewCallback(func(_ uintptr, notification uint32, first, _ uintptr) uintptr {
        if notification == 0x11 { info := (*fileInfo)(unsafe.Pointer(first)); files = append(files, entry{text(info.Name), info.Size}); return 2 }
        if notification == 0x12 { return 1 }
        return 0
    })
    path, err := syscall.UTF16PtrFromString(os.Args[1]); if err != nil { panic(err) }
    api := syscall.NewLazyDLL("setupapi.dll").NewProc("SetupIterateCabinetW")
    ok, _, callErr := api.Call(uintptr(unsafe.Pointer(path)), 0, callback, 0)
    if ok == 0 { panic(fmt.Sprintf("cabinet iteration: %v", callErr)) }
    output, err := os.OpenFile(os.Args[2], os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600); if err != nil { panic(err) }
    defer output.Close()
    if err := json.NewEncoder(output).Encode(files); err != nil { panic(err) }
    fmt.Printf("%d cabinet members (all skipped)\n", len(files))
}
