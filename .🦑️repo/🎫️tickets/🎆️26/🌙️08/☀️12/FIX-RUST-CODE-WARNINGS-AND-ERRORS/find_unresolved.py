import os
import re

db_dir = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db"

unresolved_patterns = [
    (r"\buse db_([a_z0-9_]+)::", r"use crate::db_\1::"),
    (r"\buse db_([a_z0-9_]+);", r"use crate::db_\1;"),
    (r"\buse {check_len", r"use crate::db_ids::{check_len"),
    (r"\buse DbError;", r"use crate::db_ids::DbError;"),
    (r"\buse DbCapabilities", r"use crate::db_ids::DbCapabilities"),
    (r"\bpub use db_([a_z0-9_]+)::", r"pub use crate::db_\1::"),
    (r"\bpub use db_([a_z0-9_]+);", r"pub use crate::db_\1;"),
    (r"\bpub use {DbCapabilities", r"pub use crate::db_ids::{DbCapabilities"),
]

modified_files = []

for root, dirs, files in os.walk(db_dir):
    for f in files:
        if f.endswith(".rs"):
            path = os.path.join(root, f)
            with open(path, "r", encoding="utf-8") as file:
                content = file.read()
            
            new_content = content
            for pat, repl in unresolved_patterns:
                new_content = re.sub(pat, repl, new_content)
            
            if new_content != content:
                with open(path, "w", encoding="utf-8") as file:
                    file.write(new_content)
                modified_files.append(path)

print(f"Modified {len(modified_files)} files in db module:")
for mf in modified_files:
    print(f"  {mf}")
