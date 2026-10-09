import sys
M = sys.argv[1]
def patch(path, old, new):
    s = open(path, encoding='utf-8', newline='').read()
    assert s.count(old) == 1, (path, old)
    open(path + ".new", 'w', encoding='utf-8', newline='').write(s.replace(old, new))
patch(M + "/Cargo.toml", '''[workspace.dependencies.semio-s-artifact-stdio-svg]''', '''[workspace.dependencies.semio-s-artifact-stdio-json]
path = "../../../🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust"

[workspace.dependencies.semio-s-artifact-stdio-svg]''')
patch(M + "/📦️packages/🦀️rust/Cargo.toml", "semio-s-artifact-stdio-ifc = { workspace = true }\n", "semio-s-artifact-stdio-ifc = { workspace = true }\nsemio-s-artifact-stdio-json = { workspace = true }\n")
