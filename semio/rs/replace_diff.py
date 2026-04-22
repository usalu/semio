with open("lib.rs", "r", encoding="utf-8") as f:
    lines = f.readlines()
start = 7606
end_excl = 8283
insert = [
    "pub mod diff {\n",
    '    include!("diff_body.rs");\n',
    "}\n",
    "\n",
    "pub mod kit_diff {\n",
    '    include!("kit_diff_body.rs");\n',
    "}\n",
    "\n",
]
out = lines[:start] + insert + lines[end_excl:]
with open("lib.rs", "w", encoding="utf-8", newline="\n") as f:
    f.writelines(out)
print("ok", len(out))
