import sys,os
HERE=os.path.dirname(os.path.abspath(__file__))
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
PUB=LIB+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication"
INSTALL=LIB+"/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json"
ROLES=PUB+"/🧪️tests/📋️roles/🔣️.json"
TESTS=[PUB+"/🧪️tests/📋️roles/🟦️.ts"]
TITLE="R6c landing (the dependency image declares the bazel module cache role that the deps-build-tools operation created, so the retirement plan no longer refuses the image roles)"
BAZEL='''          {
            "name": "bazel",
            "kind": "directory",
            "writer": {
              "en": "Bazel module cache of the Bazel dependency tasks (bazel/modules/<digest>); content-addressed, never edited in place.",
              "de": "Bazel-Modul-Cache der Bazel-Abhängigkeitsaufgaben (bazel/modules/<Digest>); inhaltsadressiert und nie an Ort und Stelle verändert."
            },
            "retention": "self-managed"
          },
'''
PY='          {\n            "name": "python",'
CASE=open(os.path.join(HERE,"parts-case.txt"),encoding="utf8").read()
FOREIGN='    {\n      "id": "foreign-directory",'
EDITS=[(INSTALL,PY,BAZEL+PY),(ROLES,FOREIGN,"    "+CASE.lstrip()+FOREIGN)]
NEWFILES={}
