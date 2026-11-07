package main

import (
    "archive/zip"
    "crypto/sha256"
    "encoding/hex"
    "fmt"
    "io"
    "os"
    "path/filepath"
    "regexp"
    "strings"
    "syscall"
    "unicode/utf16"
)

func main() {
    if len(os.Args) != 2 { panic("expected generated directory") }; root := os.Args[1]
    expected := []string{"03f56d24dd23314467144060c8ee70ca0fdd79a901b60e7d2f92f9cbe8d042cf", "40bc5dd011e16ee46698a8e36e71f4c85ee34dbe938368b295ed27690c569860", "428bcda4a867056ab696aeca95b1057f5282aa9a8121481e7787fae7548efde2", "254cbe1d5d5ac71ef7aebecba9980d530d3b3f63984d43bd5aca66b22d19ae30", "2ed606cdc1cb7e5d311ec19364b7df9e9932817ebfed761ae358cd438936dd57"}
    invalid := regexp.MustCompile(`[<>:"\\|?*\x00-\x1f]`); reserved := regexp.MustCompile(`(?i)^(con|prn|aux|nul|com[1-9]|lpt[1-9])(\.|$)`)
    dest, err := os.MkdirTemp(root, "msvc-stage-"); if err != nil { panic(err) }; seen := map[string]bool{}; count := 0; var total uint64
    for index, hash := range expected {
        archive := filepath.Join(root, fmt.Sprintf("msvc-%d.vsix", index)); path, err := syscall.UTF16PtrFromString(archive); if err != nil { panic(err) }
        handle, err := syscall.CreateFile(path, syscall.GENERIC_READ, syscall.FILE_SHARE_READ, nil, syscall.OPEN_EXISTING, syscall.FILE_ATTRIBUTE_NORMAL, 0); if err != nil { panic(err) }
        input := os.NewFile(uintptr(handle), archive); if input == nil { panic("open VSIX") }; info, err := input.Stat(); if err != nil || info.Size() > 60_000_000 { panic("VSIX size") }
        digest := sha256.New(); if _, err := io.Copy(digest, input); err != nil { panic(err) }; if hex.EncodeToString(digest.Sum(nil)) != hash { panic("VSIX digest mismatch") }
        z, err := zip.NewReader(input, info.Size()); if err != nil { panic(err) }
        for _, f := range z.File {
            if !strings.HasPrefix(f.Name, "Contents/") || f.FileInfo().IsDir() { continue }
            name := strings.TrimPrefix(f.Name, "Contents/"); parts := strings.Split(name, "/")
            for _, part := range parts { if part == "" || part == "." || part == ".." || invalid.MatchString(part) || reserved.MatchString(part) || strings.HasSuffix(part, ".") || strings.HasSuffix(part, " ") { panic("unsafe VSIX member") } }
            key := strings.ToLower(name); if seen[key] || f.Mode() & os.ModeType != 0 || f.UncompressedSize64 > 100_000_000 { panic("invalid VSIX member") }; seen[key] = true
            target := filepath.Join(append([]string{dest}, parts...)...); if len(utf16.Encode([]rune(target))) > 256 { panic("VSIX destination too long") }
            if err := os.MkdirAll(filepath.Dir(target), 0700); err != nil { panic(err) }
            in, err := f.Open(); if err != nil { panic(err) }; out, err := os.OpenFile(target, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0600); if err != nil { panic(err) }
            n, err := io.Copy(out, io.LimitReader(in, int64(f.UncompressedSize64)+1)); if err != nil { panic(err) }; if err := in.Close(); err != nil { panic(err) }; if err := out.Close(); err != nil { panic(err) }
            if uint64(n) != f.UncompressedSize64 { panic("VSIX member size mismatch") }; count++; total += uint64(n)
        }
        if err := input.Close(); err != nil { panic(err) }; fmt.Printf("Extracted verified VSIX %d\n", index)
    }
    if count != 494 || total != 399702857 { panic("VSIX inventory mismatch") }
    fmt.Printf("Verified %d members in %s\n", count, dest)
}
