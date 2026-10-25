import json
src=json.load(open("🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧪️tests/🧾️refusals/🔣️.json",encoding="utf8"))["texts"]
def transcript(body,task="@teaching/architecture-course:typecheck"):
    return ("\n> nx run @teaching/architecture-course:deps-javascript  [local cache]\n\n"
     "> nx run %s\n\n"
     "%s: building\n%s\n\n"
     " NX   Running target typecheck for project @teaching/architecture-course and 3 task(s) they depend on failed\n\n"
     "Failed tasks:\n\n- %s\n\nHint: run the command with --verbose for more details.\n")%(task,task,"\n".join(task+": "+line for line in body.split("\n")),task)
cases=[]
for row in src:
    cases.append({"id":"nx-"+row["id"],"text":transcript(row["text"]),"class":row["class"],"disposition":row["disposition"]})
cases.append({"id":"nx-plain-type-error","text":transcript("error TS2322: Type 'string' is not assignable to type 'number'."),"class":None,"disposition":None})
cases.append({"id":"nx-no-output","text":"","class":None,"disposition":None})
cases.append({"id":"nx-lockfile-frozen","text":transcript("error: lockfile had changes, but lockfile is frozen"),"class":None,"disposition":None})
json.dump({"version":1,"label":"@teaching/architecture-course:typecheck","texts":cases},open("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🪵️failure-tail/🧪️tests/🔣️.json","w",encoding="utf8"),ensure_ascii=False,indent=2)
open("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🧶️nx/🪵️failure-tail/🧪️tests/🔣️.json","a").write("\n")
print(len(cases))
