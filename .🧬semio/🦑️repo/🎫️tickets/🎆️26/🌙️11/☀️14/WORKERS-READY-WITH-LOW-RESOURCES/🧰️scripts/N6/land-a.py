import sys
B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
def patch(path,old,new):
    s=open(path,encoding="utf8").read()
    if s.count(old)!=1: sys.exit("anchor count %d in %s"%(s.count(old),path))
    open(path,"w",encoding="utf8").write(s.replace(old,new))
patch(B+"🕰️generations/🗃️row-log/🟨️.mjs",
 "    addList(other){if(other.ids===undefined)throw Error(\"Row log list required\");for(const id of other.ids)if(mark(id)){ids=grown(ids,count+1);ids[count++]=id;}},\n",
 "    addList(other){if(other.ids===undefined)throw Error(\"Row log list required\");for(const id of other.ids)if(mark(id)){ids=grown(ids,count+1);ids[count++]=id;}},\n    exclude(other){if(other.ids===undefined)throw Error(\"Row log list required\");for(const id of other.ids)mark(id);},\n")
patch(B+"🕰️generations/🗃️row-log/🟨️.d.mts","addList(list:RowLogListV1<Row>):void;","addList(list:RowLogListV1<Row>):void;exclude(list:RowLogListV1<Row>):void;")
patch(B+"📥️inference/🧭️script-inputs/🟨️.mjs",
 "      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe); values.set(key, entry.value); addRows(entry.rows); for (const layer",
 "      const entries = [];\n      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe); values.set(key, entry.value); entries.push(entry); for (const layer")
patch(B+"📥️inference/🧭️script-inputs/🟨️.mjs",
 "      for (const layer of layers.values()) { if (layer.fact) continue; if (!layer.members",
 "      for (const { support } of derived.values()) { if (builder) builder.exclude(support); else for (const row of support) seen.add(rowKeyV1(row)); }\n      for (const entry of entries) addRows(entry.rows);\n      for (const layer of layers.values()) { if (layer.fact) continue; if (!layer.members")
print("ok")
