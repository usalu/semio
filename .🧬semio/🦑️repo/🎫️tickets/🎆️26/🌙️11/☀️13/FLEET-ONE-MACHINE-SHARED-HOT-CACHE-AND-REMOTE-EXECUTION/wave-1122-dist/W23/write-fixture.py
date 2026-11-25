import json,os
out="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🪪️windows-facts/🔣️.json"
os.makedirs(os.path.dirname(out),exist_ok=True)
styling="C:\\semio\\🧰️framework\\🔨️modules\\🖱️ui\\🎨️styling\\📦️packages\\🟦️typescript"
pack="C:\\semio\\🧰️framework\\🔨️modules\\🎒️pack\\📦️packages\\🟦️typescript\\node_modules"
deep="C:\\deep\\"+"x"*350+"\\🟦️typescript"
tail="…"+"".join(list(deep)[len(list(deep))-299:])
base={"dev":"3221576242","ino":"281474976743925","mode":"16895","size":"0","nlink":"1","mtimeNs":"1763832720123456700","ctimeNs":"1763832720123456700","kind":"directory","volumeSerial":"c0055a32","fileId":"000000000000000000010000000081f5","attributes":16,"security":"9b1c6a0e4f7d2c8b5a3e1f0d9c8b7a6e5d4c3b2a1f0e9d8c7b6a5f4e3d2c1b0a","streams":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}
rows=[
 {"id":"ntfs-change-time-moved","refusal":"directory","directory":styling,"actualChanges":{"ctimeNs":"1764061929456789000"},
  "differences":[{"field":"ctimeNs","expected":"1763832720123456700","actual":"1764061929456789000"}],
  "texts":{"en":"Original Windows relative directory facts changed at "+styling+": ctimeNs expected 1763832720123456700, actual 1764061929456789000",
           "de":"Die ursprünglichen Fakten des relativen Windows-Ordners haben sich geändert bei "+styling+": ctimeNs erwartet 1763832720123456700, tatsächlich 1764061929456789000"}},
 {"id":"write-and-change-time-moved","refusal":"enumeration","directory":pack,"actualChanges":{"mtimeNs":"1764061929000000000","ctimeNs":"1764061929000000100"},
  "differences":[{"field":"mtimeNs","expected":"1763832720123456700","actual":"1764061929000000000"},{"field":"ctimeNs","expected":"1763832720123456700","actual":"1764061929000000100"}],
  "texts":{"en":"Windows relative enumeration changed at "+pack+": mtimeNs expected 1763832720123456700, actual 1764061929000000000; ctimeNs expected 1763832720123456700, actual 1764061929000000100",
           "de":"Die relative Windows-Aufzählung hat sich geändert bei "+pack+": mtimeNs erwartet 1763832720123456700, tatsächlich 1764061929000000000; ctimeNs erwartet 1763832720123456700, tatsächlich 1764061929000000100"}},
 {"id":"attributes-not-content-indexed","refusal":"pathParent","directory":pack,"actualChanges":{"attributes":8208},
  "differences":[{"field":"attributes","expected":"0x10","actual":"0x2010"}],
  "texts":{"en":"Current Windows relative path parent changed at "+pack+": attributes expected 0x10, actual 0x2010",
           "de":"Der aktuelle Elternordner des relativen Windows-Pfads hat sich geändert bei "+pack+": attributes erwartet 0x10, tatsächlich 0x2010"}},
 {"id":"path-entry-links-and-security","refusal":"pathDescriptor","role":"entry","directory":pack,"actualChanges":{"nlink":"2","security":"00ff"},
  "differences":[{"field":"entry.nlink","expected":"1","actual":"2"},{"field":"entry.security","expected":base["security"],"actual":"00ff"}],
  "texts":{"en":"Current exact Windows relative path descriptor changed at "+pack+": entry.nlink expected 1, actual 2; entry.security expected "+base["security"]+", actual 00ff",
           "de":"Der aktuelle exakte Deskriptor des relativen Windows-Pfads hat sich geändert bei "+pack+": entry.nlink erwartet 1, tatsächlich 2; entry.security erwartet "+base["security"]+", tatsächlich 00ff"}},
 {"id":"parent-target-appears-with-control","refusal":"pathDescriptor","role":"parent","directory":pack,"actualChanges":{"target":"\\??\\C:\\x\u0001y"},
  "differences":[{"field":"parent.target","expected":"∅","actual":"\\??\\C:\\x?y"}],
  "texts":{"en":"Current exact Windows relative path descriptor changed at "+pack+": parent.target expected ∅, actual \\??\\C:\\x?y",
           "de":"Der aktuelle exakte Deskriptor des relativen Windows-Pfads hat sich geändert bei "+pack+": parent.target erwartet ∅, tatsächlich \\??\\C:\\x?y"}},
 {"id":"long-directory-keeps-tail","refusal":"entry","directory":deep,"actualChanges":{"fileId":"000000000000000000010000000081f6"},
  "differences":[{"field":"fileId","expected":"000000000000000000010000000081f5","actual":"000000000000000000010000000081f6"}],
  "texts":{"en":"Windows relative entry changed while observed at "+tail+": fileId expected 000000000000000000010000000081f5, actual 000000000000000000010000000081f6",
           "de":"Der relative Windows-Eintrag hat sich während der Beobachtung geändert bei "+tail+": fileId erwartet 000000000000000000010000000081f5, tatsächlich 000000000000000000010000000081f6"}},
 {"id":"quiet-directory-same","refusal":"directory","directory":styling,"actualChanges":{},"differences":[],"texts":None},
]
json.dump({"base":base,"rows":rows},open(out,"w",encoding="utf8"),ensure_ascii=False,indent=1)
print(out)
