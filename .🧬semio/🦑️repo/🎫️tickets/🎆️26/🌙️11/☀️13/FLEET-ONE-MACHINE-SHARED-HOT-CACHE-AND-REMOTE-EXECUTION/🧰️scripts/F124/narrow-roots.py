import os
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers/"
path=L+"📒️allowlist/🔣️.json";s=open(path,encoding="utf8").read()
old='  "\\\\brepositoryCacheBudgetV1\\\\s*\\\\("\n'
assert s.count(old)==1
s=s.replace(old,'  "\\\\brepositoryCacheBudgetV1\\\\s*\\\\([^()]*\\\\)\\\\s*\\\\.partitions\\\\b"\n')
old='  "\\\\b(CachePartitionBudgetsV1|RepositoryCacheBudgetV1|RepositoryCachePartitionV1|DedupeRequestV1)\\\\b"\n'
assert s.count(old)==1
s=s.replace(old,'  "\\\\b(CachePartitionBudgetsV1|RepositoryCachePartitionV1|DedupeRequestV1)\\\\b"\n')
tmp=path+".f124tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,path);print("narrowed")
