const fs=require("fs");const R=process.argv[2];
function edit(f,fn){const p=R+"/"+f;fs.writeFileSync(p,fn(fs.readFileSync(p,"utf8")))}
function once(s,a,b){if(s.split(a).length!==2){console.error("count:",a);process.exit(1)}return s.replace(a,()=>b)}
edit("📋️repl-panes/🟦️.tsx",s=>{
 s=once(s,'import { type SelectionTarget }','import { useCadReplLabels } from "../🗣️labels/🟦️.ts";\nimport { type SelectionTarget }');
 s=once(s,'SelectionAttributesPaneProps): ReactNode {\n','SelectionAttributesPaneProps): ReactNode {\n  const labels = useCadReplLabels();\n');
 s=once(s,'ModelStatsPaneProps): ReactNode {\n','ModelStatsPaneProps): ReactNode {\n  const labels = useCadReplLabels();\n');
 s=once(s,'SelectionPropertiesPaneProps): ReactNode {\n','SelectionPropertiesPaneProps): ReactNode {\n  const labels = useCadReplLabels();\n');
 s=once(s,'onClick={() => clearField(defn)}>\n            Clear\n','aria-label={labels.clearField(defn.label)} onClick={() => clearField(defn)}>\n            {labels.clear}\n');
 s=once(s,'font-semibold">Attributes</span>','font-semibold">{labels.attributes}</span>');
 s=once(s,'<span>Enabled</span>','<span>{labels.enabled}</span>');
 s=once(s,'font-semibold">Stats</span>','font-semibold">{labels.stats}</span>');
 s=once(s,'text-2xs">Model</span>','text-2xs">{labels.model}</span>');
 s=once(s,'font-semibold">Properties</span>','font-semibold">{labels.properties}</span>');
 s=s.split('` · ${count} selected`').join('` · ${labels.selected(count)}`');
 return s});
edit("🎛️repl/🟦️.tsx",s=>{
 s=once(s,'import { canvasPickTargetKey } from "@semio-tech/framework";','import { canvasPickTargetKey } from "@semio-tech/framework";\nimport { useCadReplLabels } from "../🗣️labels/🟦️.ts";');
 s=once(s,'export function InteractionRepl(props: InteractionReplProps): ReactNode {\n','export function InteractionRepl(props: InteractionReplProps): ReactNode {\n  const labels = useCadReplLabels();\n');
 s=once(s,'font-semibold">Editor</strong>','font-semibold">{labels.editor}</strong>');
 s=once(s,'<span>Model definition</span>','<span>{labels.modelDefinition}</span>');
 s=once(s,'<span>Transfer from</span>','<span>{labels.transferFrom}</span>');
 s=once(s,'<span>Transfer to</span>','<span>{labels.transferTo}</span>');
 s=once(s,'aria-label="Show primitives"','aria-label={labels.showPrimitives}');
 s=once(s,'aria-label="Filter primitives"','aria-label={labels.filterPrimitives}');
 s=once(s,'aria-label="Show typologies"','aria-label={labels.showTypologies}');
 s=once(s,'aria-label="Selection typologies"','aria-label={labels.selectionTypologies}');
 s=once(s,'text-foreground">Last response</strong>','text-foreground">{labels.lastResponse}</strong>');
 return s});
