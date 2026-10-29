# Per-crate error summary from a cargo check log.
# Buffers error messages and their first "-->" location; flushes them onto the crate named by the next "could not compile" line.
function shortloc(s,   p) { sub(/^ *--> /, "", s); p = index(s, "/🧰️framework/"); if (p == 0) p = index(s, "/✏️s/"); if (p == 0) p = index(s, "/🌎️hub/"); if (p > 0) s = substr(s, p + 1); return s }
/^(error\[E[0-9]+\]|error: )/ && !/could not compile/ && !/^error: aborting/ {
  if (msg != "") { pending[++np] = msg "\t" loc }
  msg = $0; loc = ""; next
}
/^ *--> / && msg != "" && loc == "" { loc = shortloc($0) }
/^error: could not compile/ {
  if (msg != "") { pending[++np] = msg "\t" loc; msg = "" }
  crate = $0; sub(/^error: could not compile `/, "", crate); sub(/`.*/, "", crate)
  cnt = np
  printf "CRATE\t%s\t%d\n", crate, cnt
  for (i = 1; i <= np; i++) printf "ERR\t%s\t%s\n", pending[i], ""
  np = 0
  next
}
END {
  if (msg != "") { pending[++np] = msg "\t" loc }
  if (np > 0) { printf "CRATE\t(unattributed)\t%d\n", np; for (i = 1; i <= np; i++) printf "ERR\t%s\t\n", pending[i] }
}
