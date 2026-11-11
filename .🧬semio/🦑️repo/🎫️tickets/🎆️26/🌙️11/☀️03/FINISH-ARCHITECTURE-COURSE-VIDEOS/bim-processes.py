import psutil,json
p=psutil.Process(24912)
a=p.cmdline()
assert all(x in a for x in ("bim_information","bim_objects","bim_representations","bim_models","bim_exchange","bim_consistency"))
children=p.children(recursive=True)
assert {7484,52840,48776,66156} == {q.pid for q in children}
assert all("manim" in q.cmdline() for q in children)
for q in [p,*children]:
    try: q.terminate()
    except psutil.NoSuchProcess: pass
gone,alive=psutil.wait_procs([p,*children],timeout=10)
print(json.dumps({"stopped":[q.pid for q in gone],"alive":[q.pid for q in alive]}),flush=True)
