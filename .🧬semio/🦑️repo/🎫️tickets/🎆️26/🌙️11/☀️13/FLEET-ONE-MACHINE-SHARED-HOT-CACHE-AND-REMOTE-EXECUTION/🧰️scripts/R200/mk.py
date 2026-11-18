import sys,shutil
LIVE="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
shutil.copy(LIVE,"si.base")
t=open("si.base",encoding="utf8").read()
def sub(old,new):
    global t
    assert t.count(old)==1,(t.count(old),old[:60])
    t=t.replace(old,new)
sub('''  if (files === undefined) groups.set(name, files = new Set((await toolClosureGroupV1(workspaceRoot, scripts, name)).filter(item => typeof item === "string")));
  return files;''','''  if (files === undefined) { const pending = toolClosureGroupV1(workspaceRoot, scripts, name).then(list => new Set(list.filter(item => typeof item === "string"))); groups.set(name, pending); pending.catch(() => { if (groups.get(name) === pending) groups.delete(name); }); files = pending; }
  return files;''')
sub("/** 🫙️ The files of the group of one declared tool command, once per script input cache.","/** 🫙️ The files of the group of one declared tool command, once per script input cache, however many targets ask while the first derivation runs.")
sub('''scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); }''','''scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); scripts.groupLists?.clear(); }''')
sub('''scripts.commandDependencies.clear(); scripts.declared.clear();''','''scripts.commandDependencies.clear(); scripts.groupLists?.clear(); scripts.declared.clear();''')
sub('''for (const item of await produceClosureRecipeV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item);''','''for (const item of await sharedGroupV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item);''')
sub('''/** 📇️ A command input list (or a target-sources value) with every shared reference''','''/** 🗄️ The group a shared reference stands for (a tool closure or a script provider of the providers project), derived once per script input cache and replayed with its observed rows into every project that expands it again; the 17 targets of a crate expanded the same 18 groups 690 times, each time walking the 1800 modules of the cargo command again. A group that is being derived is awaited, then replayed, so concurrent expansions neither derive twice nor lose the rows. Any other recipe is produced as before.
 * @see ./🧪️tests/🗂️shared-groups/🟦️.ts */
async function sharedGroupV1(recipe, workspaceRoot, scripts) {
  if (recipe.fn !== "tool-closure" && recipe.fn !== "script-provider") return produceClosureRecipeV1(recipe, workspaceRoot, scripts);
  let index = 0;
  const key = closureRecipeKeyV1(recipe), cache = scripts.groupLists ??= new Map(), flights = scripts.groupFlights ??= new Map();
  const run = () => memoizedObservedAsyncComputationV1(cache, key, () => { scripts.groupComputations = (scripts.groupComputations ?? 0) + 1; return produceClosureRecipeV1(recipe, workspaceRoot, scripts); }, (force = false) => yieldInferenceV1(scripts.context, "build:command-source-packages", ++index, force));
  const flight = flights.get(key);
  if (flight !== undefined) { await flight.catch(() => {}); return run(); }
  const mine = run();
  flights.set(key, mine);
  try { return await mine; } finally { if (flights.get(key) === mine) flights.delete(key); }
}

/** 📇️ A command input list (or a target-sources value) with every shared reference''')
open("si.new.mjs","w",encoding="utf8").write(t)
print("ok",len(t))
