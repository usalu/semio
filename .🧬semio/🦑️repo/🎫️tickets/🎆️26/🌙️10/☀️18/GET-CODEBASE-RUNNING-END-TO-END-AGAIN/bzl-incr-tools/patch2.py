import re,sys
C=sys.argv[1]
changes={
 "runner.ts":[("Everything the real ports need","🏠"),("Copies tracked files","🪞"),("Selects scenarios by id","🔖"),("Reads a dry-run plan","📃"),("Builds the slice of a target","🔪"),("The part of the Bazel adapter","🧩"),("The real ports","🔗")],
 "command.ts":[("The closed arguments","📝"),("Reads `incremental list","🧷"),("Renders the expected re-execution sets","🗺"),("Everything the command needs","🎛"),("Runs `repo incremental`","🚀")],
 "model.ts":[("Looks up the output effect","🔬"),("Admits a slice only when","🛡"),("Renders one mutation","🖋"),("Evaluates the tasks a tool re-executes","🧮"),("Judges the executed tasks","🔦"),("Renders the scenario list","📑"),("Tallies judged rows","🔢")],
 "observers.ts":[("Maps Bazel spawns","🔭"),("Maps the run record Nx","🗒"),("Reads the spawns","🪡")],
 "test.ts":[("Incremental build conformance:","🔁")],
}
for f,rows in changes.items():
    p=C+"/bzl-incr-tools/src/"+f;s=open(p,encoding="utf8").read()
    for start,emoji in rows:
        pat=re.compile(r"/\*\* \S️? "+re.escape(start))
        assert len(pat.findall(s))==1,(f,start,len(pat.findall(s)))
        s=pat.sub("/** "+emoji+" "+start,s)
    open(p,"w",encoding="utf8").write(s)
print("done")
