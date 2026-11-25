import os,sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def sub(a,b):
    global s
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
sub('''/** 🧾️ The identity a persisted holder record names, a hard error when `pid` or `startMs` is missing or not a positive integer. Holders rewrite their records on their next start; there is no pid-only path.''',
'''/** 🧾️ The identity a persisted holder record names: a hard error when `pid` is not a positive integer or `startMs` is present and not a positive integer. A record that has no `startMs` at all was written before the creation time was recorded (live machines hold such records once): it reads as `startMs` 0 (start unknown, only the pid counts) and the holder writes `startMs` with its next write; no writer ever produces it.''')
sub('''!Number.isSafeInteger(row.startMs) || (row.startMs as number) < 1) throw new ProcessIdentityRecordErrorV1(where);
  return {pid: row.pid as number, startMs: row.startMs as number};''',
'''(row.startMs !== undefined && (!Number.isSafeInteger(row.startMs) || (row.startMs as number) < 1))) throw new ProcessIdentityRecordErrorV1(where);
  return {pid: row.pid as number, startMs: row.startMs === undefined ? 0 : row.startMs as number};''')
sub('''  if (seen === "unknown") return "unknown";
  return Math.abs''','''  if (seen === "unknown") return "unknown";
  if (identity.startMs === 0) return "same";
  return Math.abs''')
sub('''A record without a creation time is a hard error.
 * @see ./🧪️tests''','''A record without a creation time (`startMs` 0, written before it was recorded) is judged by its pid alone.
 * @see ./🧪️tests''')
t=p+".f45tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,p)
