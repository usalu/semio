package main

import (
    "crypto/sha256"
    "encoding/hex"
    "fmt"
    "os"
    "path/filepath"
    "syscall"
    "unsafe"
)

type fileInfo struct { Name *uint16; Size uint32; Error uint32; Date uint16; Time uint16; Attributes uint16; Target [260]uint16 }
func text(p *uint16) string {
    if p == nil { panic("missing name") }; data := make([]uint16, 0, 128)
    for i := uintptr(0); i < 128; i++ { c := *(*uint16)(unsafe.Add(unsafe.Pointer(p), i*2)); if c == 0 { return syscall.UTF16ToString(data) }; data = append(data, c) }
    panic("name too long")
}
func main() {
    if len(os.Args) != 2 { panic("expected owned ASCII alias") }
    root := os.Args[1]; cab := filepath.Join(root, "26ea25236f12b23db661acf268a70cfa.cab")
    raw, err := os.ReadFile(cab); if err != nil { panic(err) }; sum := sha256.Sum256(raw)
    if len(raw) != 93999 || hex.EncodeToString(sum[:]) != "a6d81007e50b1881d58e802ef4402ff16c62ff4960b41c4263057e3173981919" { panic("cabinet integrity mismatch") }
    dest := filepath.Join(root, "cab-sample-output"); if err := os.Mkdir(dest, 0700); err != nil { panic(err) }
    expected := map[string]uint32{"cat26ea25236f12b23db661acf268a70cfa": 11124, "fil21adcb655557a8d1eb06109c707690b4": 166232}; seen := map[string]bool{}
    callback := syscall.NewCallback(func(_ uintptr, notification uint32, first, _ uintptr) uintptr {
        if notification == 0x11 {
            info := (*fileInfo)(unsafe.Pointer(first)); name := text(info.Name); size, ok := expected[name]
            if !ok || seen[name] || size != info.Size { return 0 }
            target := filepath.Join(dest, name); encoded, err := syscall.UTF16FromString(target)
            if err != nil || len(encoded) > len(info.Target) { return 0 }
            copy(info.Target[:], encoded); seen[name] = true; return 1
        }
        if notification == 0x12 { return 1 }; return 0
    })
    path, err := syscall.UTF16PtrFromString(cab); if err != nil { panic(err) }
    ok, _, callErr := syscall.NewLazyDLL("setupapi.dll").NewProc("SetupIterateCabinetW").Call(uintptr(unsafe.Pointer(path)), 0, callback, 0)
    if ok == 0 || len(seen) != len(expected) { panic(fmt.Sprintf("cabinet extraction failed: %v", callErr)) }
    for name, size := range expected { stat, err := os.Stat(filepath.Join(dest, name)); if err != nil || stat.Size() != int64(size) { panic("extracted size mismatch") } }
    fmt.Printf("%d verified cabinet members extracted into owned sample directory\n", len(seen))
}
