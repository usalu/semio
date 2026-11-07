package main
import ("archive/zip"; "crypto/sha256"; "encoding/hex"; "encoding/json"; "fmt"; "os"; "strings")
type entry struct { Name string `json:"name"` }
func main() {
    if len(os.Args) != 3 { panic("expected archive and inventory") }; b, err := os.ReadFile(os.Args[1]); if err != nil { panic(err) }; hash := sha256.Sum256(b)
    if hex.EncodeToString(hash[:]) != "2ed606cdc1cb7e5d311ec19364b7df9e9932817ebfed761ae358cd438936dd57" { panic("archive hash mismatch") }
    z, err := zip.OpenReader(os.Args[1]); if err != nil { panic(err) }; defer z.Close()
    raw, err := os.ReadFile(os.Args[2]); if err != nil { panic(err) }; var existing []entry; if err := json.Unmarshal(raw, &existing); err != nil { panic(err) }; names := map[string]bool{}; for _, x := range existing { names[strings.ToLower(x.Name)] = true }
    total, duplicate := 0, 0; var bytes uint64
    for _, f := range z.File { if strings.HasPrefix(f.Name, "Contents/") && !f.FileInfo().IsDir() { total++; bytes += f.UncompressedSize64; if names[strings.ToLower(strings.TrimPrefix(f.Name, "Contents/"))] { duplicate++; fmt.Println("duplicate", f.Name) }; if strings.Contains(strings.ToLower(f.Name), "oldnames") { fmt.Printf("%s %d\n", f.Name, f.UncompressedSize64) } } }
    fmt.Printf("%d members, %d bytes, %d duplicate paths\n", total, bytes, duplicate)
}
