GLUE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs"
glue = open(GLUE, encoding="utf-8").read()
bad = '                                #[path = "."]\n                                #[path = "."]\n'
good = '                                #[path = "."]\n'
n = glue.count(bad)
print("dup occurrences:", n)
glue = glue.replace(bad, good)
open(GLUE, "w", encoding="utf-8").write(glue)
