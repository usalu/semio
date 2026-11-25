import re,sys,hashlib
R="/Users/ueli/Documents/semio/🧰️framework/"
O=R+"🔨️modules/🌱️value/🛬️decode/🧳️owned/"
files={
O+"🏗️record-machine/🦀️.rs":[
 (r'#\[path="[^"\n]*debug-ledger\.rs"\]#\[allow\(dead_code\)\]pub\(crate\) mod dbg;\n',''),
 (r'\n if dbg::on\(\)\{dbg::add\(format!\("leaf:[^\n]*\n','\n'),
 (r'thread_local!\{static DBG_INNER[^\n]*\nfn close_poll<[^\n]*\n if !dbg::on\(\)[^\n]*\n let before=[^\n]*\n if !inner\{[^\n]*\n poll\n\}\nfn close_poll_inner<','fn close_poll<'),
],
O+"🏗️record-machine/🔀️enum/🦀️.rs":[
 (r'#\[path="[^"\n]*debug-ledger\.rs"\]#\[allow\(dead_code\)\]pub\(crate\) mod dbg;\n',''),
 (r'   if dbg::on\(\)\{dbg::add\(format!\("enum:[^\n]*\n',''),
],
O+"🧳️original/🦀️.rs":[
 (r'#\[path="[^"\n]*debug-ledger\.rs"\]#\[allow\(dead_code\)\]mod dbg;\n',''),
 (r'if dbg::on\(\)\{dbg::add\("recvroot"\.into\(\),turn\.work,progress\.copied_items as u128,progress\.copied_bytes as u128\)\}',''),
 (r'  let dbg_branch=if dbg::on\(\)[^\n]*\n',''),
 (r'if let Some\(b\)=dbg_branch\{let\(i,y\)=match &result\{[^\n]*?dbg::add\(format!\("closeroot:\{b\}"\),work,i,y\)\}',''),
],
O+"🧬️protocol/📦️packages/🦀️rust/🦀️.rs":[
 (r'#\[path="[^"\n]*debug-ledger\.rs"\]#\[allow\(dead_code\)\]mod dbg;\n',''),
 (r'if dbg::on\(\)&&self\.demand\.is_none\(\)\{let n=dbg::strip.*?dbg::add\(format!\("poll:\{n\}:d\{\}",demand\.copy_bytes\),work as u128,1,demand\.copy_bytes as u128\)\}',''),
],
O+"📦️storage/🦀️.rs":[
 (r'#\[path="[^"\n]*debug-ledger\.rs"\]#\[allow\(dead_code\)\]mod dbg;\n',''),
 (r'if dbg::on\(\)\{dbg::add\(format!\("seeddemand:[^\n]*?std::mem::size_of::<T>\(\) as u128\)\}',''),
],
R+"🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🫴️record/🦀️.rs":[
 (r'#\[path="[^"\n]*debug-ledger\.rs"\]#\[allow\(dead_code\)\]mod dbg;\n',''),
 (r'  if dbg::on\(\)\{let w=cx\.original_work_units_spent[^\n]*\n',''),
],
}
for p,subs in files.items():
    s=open(p,encoding='utf-8').read();b=hashlib.sha256(s.encode()).hexdigest()[:8]
    for pat,rep in subs:
        s,n=re.subn(pat,rep,s,flags=re.S if 'dbg::strip' in pat else 0)
        assert n==1,(p,pat[:40],n)
    assert 'dbg' not in s.replace('dbg_',''),(p,[m.group(0) for m in re.finditer(r'.{30}dbg.{30}',s)])
    if '--write' in sys.argv:
        open(p,'w',encoding='utf-8').write(s);print('wrote',p.split('/')[-3],b,'->',hashlib.sha256(s.encode()).hexdigest()[:8])
    else: print('ok',p.split('/')[-3])
