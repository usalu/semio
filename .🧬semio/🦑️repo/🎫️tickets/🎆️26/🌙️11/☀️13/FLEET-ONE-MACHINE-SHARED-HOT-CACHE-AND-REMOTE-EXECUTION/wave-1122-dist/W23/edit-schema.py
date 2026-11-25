import os,sys,json
path="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json"
src=open(path,encoding="utf8").read()
anchor='    "RelativeWindowsFailureNames": {\n'
assert src.count(anchor)==1 and '"RelativeWindowsFactsChange"' not in src
block='''    "RelativeWindowsFactsChange": {
      "const": {
        "version": 1,
        "identityFields": ["dev", "ino", "mode", "size", "nlink", "mtimeNs", "ctimeNs"],
        "fields": ["kind", "volumeSerial", "fileId", "attributes", "security", "streams", "target"],
        "hexFields": ["attributes"],
        "absent": "absent",
        "maximumDirectoryPoints": 300,
        "maximumValuePoints": 120,
        "separator": "; ",
        "refusals": {
          "directory": {"en": "Original Windows relative directory facts changed", "de": "Die ursprünglichen Fakten des relativen Windows-Ordners haben sich geändert"},
          "entry": {"en": "Windows relative entry changed while observed", "de": "Der relative Windows-Eintrag hat sich während der Beobachtung geändert"},
          "enumerationParent": {"en": "Exact Windows enumeration parent required", "de": "Der exakte Windows-Ordner der Aufzählung ist erforderlich"},
          "enumeration": {"en": "Windows relative enumeration changed", "de": "Die relative Windows-Aufzählung hat sich geändert"},
          "pathParent": {"en": "Current Windows relative path parent changed", "de": "Der aktuelle Elternordner des relativen Windows-Pfads hat sich geändert"},
          "pathDescriptor": {"en": "Current exact Windows relative path descriptor changed", "de": "Der aktuelle exakte Deskriptor des relativen Windows-Pfads hat sich geändert"}
        },
        "texts": {
          "en": {"message": "{refusal} at {directory}: {differences}", "difference": "{field} expected {expected}, actual {actual}"},
          "de": {"message": "{refusal} bei {directory}: {differences}", "difference": "{field} erwartet {expected}, tatsächlich {actual}"}
        }
      }
    },
'''
out=src.replace(anchor,block+anchor)
json.loads(out)
tmp=path+".w23.tmp"
open(tmp,"w",encoding="utf8").write(out);os.replace(tmp,path)
print("ok")
