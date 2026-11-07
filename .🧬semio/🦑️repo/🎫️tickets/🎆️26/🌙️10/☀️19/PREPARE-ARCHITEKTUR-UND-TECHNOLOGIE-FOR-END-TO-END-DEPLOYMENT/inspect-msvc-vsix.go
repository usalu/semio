package main

import (
    "archive/zip"
    "crypto/sha256"
    "encoding/hex"
    "encoding/json"
    "fmt"
    "io"
    "os"
    "path/filepath"
    "strings"
    "syscall"
)
type entry struct { Archive int `json:"archive"`; Name string `json:"name"`; Size uint64 `json:"size"`; Mode uint32 `json:"mode"` }
func main() {
    if len(os.Args) != 2 { panic("expected generated directory") }; root := os.Args[1]
    expected := []string{"03f56d24dd23314467144060c8ee70ca0fdd79a901b60e7d2f92f9cbe8d042cf", "40bc5dd011e16ee46698a8e36e71f4c85ee34dbe938368b295ed27690c569860", "428bcda4a867056ab696aeca95b1057f5282aa9a8121481e7787fae7548efde2", "254cbe1d5d5ac71ef7aebecba9980d530d3b3f63984d43bd5aca66b22d19ae30", "2ed606cdc1cb7e5d311ec19364b7df9e9932817ebfed761ae358cd438936dd57"}
    rows := []entry{}; seen := map[string]bool{}; var total uint64
    for index, hash := range expected {
        path := filepath.Join(root, fmt.Sprintf("msvc-%d.vsix", index)); name, err := syscall.UTF16PtrFromString(path); if err != nil { panic(err) }; h, err := syscall.CreateFile(name, syscall.GENERIC_READ, syscall.FILE_SHARE_READ, nil, syscall.OPEN_EXISTING, syscall.FILE_ATTRIBUTE_NORMAL, 0); if err != nil { panic(err) }; input := os.NewFile(uintptr(h), path); if input == nil { panic("open VSIX") }; info, err := input.Stat(); if err != nil || info.Size() > 60_000_000 { panic("VSIX size") }
        digest := sha256.New(); if _, err := io.Copy(digest, input); err != nil { panic(err) }; if hex.EncodeToString(digest.Sum(nil)) != hash { panic("VSIX digest mismatch") }
        z, err := zip.NewReader(input, info.Size()); if err != nil { panic(err) }
        for _, f := range z.File {
            if !strings.HasPrefix(f.Name, "Contents/") || f.FileInfo().IsDir() { continue }
            name := strings.TrimPrefix(f.Name, "Contents/"); key := strings.ToLower(name)
            if seen[key] || f.Mode() & os.ModeType != 0 || f.UncompressedSize64 > 100_000_000 { panic("unsafe VSIX member: " + name) }; seen[key] = true; total += f.UncompressedSize64
            rows = append(rows, entry{index, name, f.UncompressedSize64, uint32(f.Mode())})
        }
        if err := input.Close(); err != nil { panic(err) }
    }
    out, err := os.OpenFile(filepath.Join(root, "msvc-vsix-inventory-5.json"), os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0600); if err != nil { panic(err) }; defer out.Close()
    if err := json.NewEncoder(out).Encode(rows); err != nil { panic(err) }
    fmt.Printf("%d members, %d bytes\n", len(rows), total)
}
