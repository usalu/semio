import sys,json
out=sys.stdout
for line in sys.stdin:
    if line.startswith("{"):
        try: j=json.loads(line)
        except Exception: out.write(line); continue
        r=j.get("reason")
        if r=="compiler-message":
            m=j["message"]
            if m.get("level") in ("error","failure-note"):
                sp=[(s["file_name"],s["line_start"],s["column_start"],s.get("is_primary")) for s in m.get("spans",[])]
                out.write(json.dumps({"E":1,"pkg":j["package_id"],"tgt":j["target"]["name"],"kind":j["target"]["kind"],"code":(m.get("code") or {}).get("code"),"msg":m["message"],"spans":sp,"rendered":(m.get("rendered") or "")[:7000]},ensure_ascii=False)+"\n")
        elif r=="build-finished":
            out.write(line)
    else:
        out.write(line)
out.flush()
