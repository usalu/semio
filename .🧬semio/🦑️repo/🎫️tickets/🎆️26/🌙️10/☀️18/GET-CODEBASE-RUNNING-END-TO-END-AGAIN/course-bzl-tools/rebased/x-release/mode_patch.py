import os,re

REPO=os.path.abspath(os.path.join(os.path.dirname(__file__),"../../../../../../../../.."))
BUILD="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
ADAPTER_DIR=os.path.join(REPO,BUILD,"🔧️tools/🏗️bazel")
ADAPTER=os.path.join(ADAPTER_DIR,"📜️script.ts")
TESTS=os.path.join(ADAPTER_DIR,"🧪️tests","📥️mode")
STAGED_TESTS=os.path.join(os.path.dirname(__file__),"tests","📥️mode")
BUILD_SCRIPT=os.path.join(REPO,BUILD,"📜️script.ts")
README=os.path.join(REPO,"🧰️framework/🛍️products/🦑️repo/README.md")

IMPORT_ANCHOR='import {scriptCompilerAdmissionControlV1,compilerAdmissionBudgetV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";\n'
IMPORT_NEW=IMPORT_ANCHOR+'import {buildModesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";\n'

IDENTITY_OLD='''/** 🧭️ Resolves task identity and its configuration without implementation-specific parsing. */
function taskIdentity(graph:BuildGraphV1,value:string,configuration?:string):{id:string;project:string;target:string;configuration?:string}{
 const parts=value.split(":"),project=parts[0]!,target=parts[1]!,definition=graph.projects[project]?.targets[target];
 if(parts.length<2||parts.length>3||!definition)throw new Error(`Unknown build task: ${value}`);
 const requested=parts[2]??configuration,selected=requested&&definition.configurations?.[requested]?requested:definition.defaultConfiguration as string|undefined,chosen=selected&&definition.configurations?.[selected]?selected:undefined;
 if(parts[2]&&!definition.configurations?.[parts[2]])throw new Error(`Unknown build configuration: ${value}`);
 return {project,target,...(chosen?{configuration:chosen}:{}),id:`${project}:${target}${chosen?":"+chosen:""}`};
}
'''
IDENTITY_NEW='''/** 🎚️ Tells whether a name is a compiler mode: prototype and release are one invocation axis that every target admits, whether or not it declares them as configurations. */
function bazelBuildModeAxisV1(name:string|undefined):name is "prototype"|"release"{
 return name!==undefined&&(buildModesV1() as readonly string[]).includes(name);
}

/** 🎚️ Tells whether a target declares no configurations at all, so that the compiler mode is its only configuration; a target that declares some keeps Nx's rule that an undeclared one falls back to its default. */
function bazelModeOnlyV1(target:BuildTargetV1):boolean{
 return target.configurations===undefined||Object.keys(target.configurations).length===0;
}

/** 🧭️ Resolves task identity and its configuration without implementation-specific parsing; a declared configuration or a compiler mode selects the task's configuration. */
function taskIdentity(graph:BuildGraphV1,value:string,configuration?:string):{id:string;project:string;target:string;configuration?:string}{
 const parts=value.split(":"),project=parts[0]!,target=parts[1]!,definition=graph.projects[project]?.targets[target];
 if(parts.length<2||parts.length>3||!definition)throw new Error(`Unknown build task: ${value}`);
 const admitted=(name:string|undefined):name is string=>name!==undefined&&(bazelModeOnlyV1(definition)&&bazelBuildModeAxisV1(name)||definition.configurations!==undefined&&Object.hasOwn(definition.configurations,name));
 const requested=parts[2]??configuration,selected=admitted(requested)?requested:definition.defaultConfiguration as string|undefined,chosen=admitted(selected)?selected:undefined;
 if(parts[2]&&!admitted(parts[2]))throw new Error(`Unknown build configuration: ${value}\\nUnbekannte Build-Konfiguration: ${value}`);
 return {project,target,...(chosen?{configuration:chosen}:{}),id:`${project}:${target}${chosen?":"+chosen:""}`};
}

/** 🪪️ Binds a task request to the mode of the original caller, so every planned task of every target carries the mode in its id, label and workspace. */
export function bazelAuthorizedRequestV1(request:BazelTaskRequestV1,invocation:ScriptInvocation):BazelTaskRequestV1{
 const mode=(readIssuedScriptProcessEnvelopeV1(invocation).capabilities as {buildMode?:string}).buildMode;
 return bazelBuildModeAxisV1(mode)?{...request,authority:mode}:request;
}

/** 🚦️ Refuses at plan time a task whose mode disagrees with the original caller or with the mode its target is fixed to. */
export function requireBazelModeAdmissionV1(id:string,configuration:string|undefined,target:BuildTargetV1,authority:string|undefined):void{
 if(!bazelBuildModeAxisV1(configuration))return;
 const fixed=(target.metadata as {semio?:{buildMode?:{fixed?:string}}}|undefined)?.semio?.buildMode?.fixed;
 if(authority!==undefined&&configuration!==authority)throw new Error(`Conflicting build modes: task ${id} selects ${configuration}, the original caller selects ${authority}\\nWidersprüchliche Build-Modi: Task ${id} wählt ${configuration}, der ursprüngliche Aufrufer wählt ${authority}`);
 if(fixed!==undefined&&fixed!==configuration)throw new Error(`Conflicting build modes: task ${id} is fixed to ${fixed} but the invocation selects ${configuration}; select ${fixed} or another target\\nWidersprüchliche Build-Modi: Task ${id} ist fest auf ${fixed}, die Auswahl ist aber ${configuration}; wähle ${fixed} oder ein anderes Target`);
}

/** 🎯️ Resolves a target for one configuration; a compiler mode of a target without configurations only exports the mode to the task environment. */
export function bazelTargetV1(target:BuildTargetV1,configuration?:string):BuildTargetV1{
 if(bazelBuildModeAxisV1(configuration)&&bazelModeOnlyV1(target))return {...target,execution:{...target.execution,env:{...target.execution.env,SEMIO_BUILD_MODE:configuration}}};
 return resolveBuildTargetV1(target,configuration);
}
'''

REQUEST_OLD='export interface BazelTaskRequestV1{tasks:string[];args?:string[];configuration?:string;'
REQUEST_NEW='export interface BazelTaskRequestV1{tasks:string[];args?:string[];configuration?:string;authority?:"prototype"|"release";'
PLAN_OLD='definition=resolveBuildTargetV1(owner.targets[target]!,identity.configuration)'
PLAN_NEW='definition=bazelTargetV1(owner.targets[target]!,identity.configuration)'
EXPLICIT_OLD='explicit=new Set(request.tasks.map(value=>taskIdentity(graph,value,request.configuration).id))'
EXPLICIT_NEW='explicit=new Set(request.tasks.map(value=>taskIdentity(graph,value,request.configuration??request.authority).id))'
SELECT_OLD='selected.push(include(value,request.args??[],request.configuration))'
SELECT_NEW='selected.push(include(value,request.args??[],request.configuration??request.authority))'
ADMIT_OLD='  if(explicit.has(id))args=request.args??[];\n'
ADMIT_NEW='  requireBazelModeAdmissionV1(id,identity.configuration,graph.projects[project]!.targets[target]!,request.authority);\n  if(explicit.has(id))args=request.args??[];\n'
RUN_OLD='phaseControl:ResourceLeaseControl,onStarted?:()=>void|Promise<void>):Promise<number>{requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);return '
RUN_NEW='phaseControl:ResourceLeaseControl,onStarted?:()=>void|Promise<void>):Promise<number>{request=bazelAuthorizedRequestV1(request,invocation);requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);return '
ORCH_OLD='base,manifest.orchestration,planBazelTasksV1,(environment,actual)'
ORCH_NEW='base,manifest.orchestration,(graph,request)=>planBazelTasksV1(graph,bazelAuthorizedRequestV1(request,invocation)),(environment,actual)'

ENV_OLD='''export function issueBazelTaskEnvironmentV1(workspace:string,invocation:ScriptInvocation,task:BazelTaskV1,environment:NodeJS.ProcessEnv):NodeJS.ProcessEnv{
 return {...issueBuildTaskEnvironmentV1(workspace,invocation,task.definition.execution,task.args,environment),...bazelTaskCargoDirectoriesV1(workspace,task)};
}
'''
ENV_NEW='''export function issueBazelTaskEnvironmentV1(workspace:string,invocation:ScriptInvocation,task:BazelTaskV1,environment:NodeJS.ProcessEnv):NodeJS.ProcessEnv{
 const issued={...issueBuildTaskEnvironmentV1(workspace,invocation,task.definition.execution,task.args,environment),...bazelTaskCargoDirectoriesV1(workspace,task)};
 if((task.configuration==="prototype"||task.configuration==="release")&&task.configuration!==issued.SEMIO_BUILD_MODE)throw new Error(`Conflicting build modes: task ${task.id} is ${task.configuration}, the original caller is ${issued.SEMIO_BUILD_MODE}\\nWidersprüchliche Build-Modi: Task ${task.id} ist ${task.configuration}, der ursprüngliche Aufrufer ist ${issued.SEMIO_BUILD_MODE}`);
 return issued;
}
'''

EDITS=[("import",IMPORT_ANCHOR,IMPORT_NEW),("request",REQUEST_OLD,REQUEST_NEW),("identity",IDENTITY_OLD,IDENTITY_NEW),("plan",PLAN_OLD,PLAN_NEW),("explicit",EXPLICIT_OLD,EXPLICIT_NEW),("select",SELECT_OLD,SELECT_NEW),("admission",ADMIT_OLD,ADMIT_NEW),("run",RUN_OLD,RUN_NEW),("orchestration",ORCH_OLD,ORCH_NEW),("environment",ENV_OLD,ENV_NEW)]

SCRIPT_OLD='adapter.planBazelTasksV1(graph,request)'
SCRIPT_NEW='adapter.planBazelTasksV1(graph,adapter.bazelAuthorizedRequestV1(request,invocation))'
SCRIPT_EDITS=[("dry-run",SCRIPT_OLD,SCRIPT_NEW)]

EN_ORIGINAL='disagreeing values are refused ("Conflicting build modes").'
DE_ORIGINAL='widersprüchliche Werte werden abgelehnt („Conflicting build modes“).'
EN_NOW=('disagreeing values are refused ("Conflicting build modes", stated in English and German, naming the fixed-mode targets such as `dev` that pin a mode). '
 'The course verbs that need release (`publish`, `test-e2e` rehearsal, `docker-image-build`, `docker-image-publish`, `deploy-check` of `@teaching/architecture-course`) read the original caller\'s mode and refuse a prototype caller in English and German; select release with `bun repo [--tool=bazel] run @teaching/architecture-course:publish:release`.')
DE_NOW=('widersprüchliche Werte werden abgelehnt („Conflicting build modes“, auf Englisch und Deutsch, mit den Targets mit festem Modus wie `dev`, die einen Modus vorgeben). '
 'Die Kurs-Verben, die release brauchen (`publish`, `test-e2e` im Probelauf, `docker-image-build`, `docker-image-publish`, `deploy-check` von `@teaching/architecture-course`), lesen den Modus des ursprünglichen Aufrufers und lehnen einen prototype-Aufrufer auf Englisch und Deutsch ab; release wählst du mit `bun repo [--tool=bazel] run @teaching/architecture-course:publish:release`.')
EN_FULL=(EN_NOW.split(' The course verbs')[0]+' The mode is one invocation axis, not a per-target setting: the Bazel implementation accepts `prototype` and `release` for every target whether or not the target declares them as configurations, every planned task of every target carries the mode in its id, label and workspace, the dependencies of the selected task inherit the mode, the original caller\'s mode is the authority every task environment exports (`SEMIO_BUILD_MODE`) and is part of every action key, and the plan refuses a task whose mode differs from the caller\'s and a dependency whose target is fixed to another mode. Only `prototype` and `release` can be selected at the command line; other configuration names exist only in the Bazel adapter. The course verbs'+EN_NOW.split(' The course verbs')[1])
DE_FULL=(DE_NOW.split(' Die Kurs-Verben')[0]+' Der Modus ist eine Achse des Aufrufs, keine Einstellung je Target: Die Bazel-Implementierung akzeptiert `prototype` und `release` für jedes Target, ob es sie als Konfigurationen deklariert oder nicht, jede geplante Task jedes Targets trägt den Modus in Id, Label und Workspace, die Abhängigkeiten der gewählten Task erben den Modus, der Modus des ursprünglichen Aufrufers ist die Autorität, die jede Task-Umgebung exportiert (`SEMIO_BUILD_MODE`), und gehört zu jedem Action-Schlüssel, und der Plan lehnt eine Task mit abweichendem Modus sowie eine Abhängigkeit ab, deren Target auf einen anderen Modus festgelegt ist. Auf der Kommandozeile lassen sich nur `prototype` und `release` wählen; andere Konfigurationsnamen gibt es nur im Bazel-Adapter. Die Kurs-Verben'+DE_NOW.split(' Die Kurs-Verben')[1])
EN_V1_MARK='The mode is one invocation axis, not a per-target setting: both tools accept'
DE_V1_MARK='Der Modus ist eine Achse des Aufrufs, keine Einstellung je Target: Beide Werkzeuge akzeptieren'

def _swap(text,original,now,full,marker,target):
    start=text.find(original) if original in text else None
    if target in text: return text
    if marker in text:
        head=text.index('disagreeing values are refused') if original==EN_ORIGINAL else text.index('widersprüchliche Werte werden abgelehnt')
        end=text.index('publish:release`.',head)+len('publish:release`.')
        return text[:head]+target+text[end:]
    if original in text: return text.replace(original,target)
    for known in (now,full):
        if known in text: return text.replace(known,target)
    raise SystemExit("refused: README mode paragraph not found in any known state")

def readme(text,state):
    now=state=="now"
    return _swap(_swap(text,EN_ORIGINAL,EN_NOW,EN_FULL,EN_V1_MARK,EN_NOW if now else EN_FULL),DE_ORIGINAL,DE_NOW,DE_FULL,DE_V1_MARK,DE_NOW if now else DE_FULL)

def patched(text,edits=None):
    for name,old,new in (EDITS if edits is None else edits):
        if new in text: continue
        n=text.count(old)
        if n!=1: raise SystemExit(f"refused: anchor '{name}' found {n} times (expected exactly 1); the target changed, rebase mode_patch.py")
        text=text.replace(old,new)
    return text

def applied(text,edits=None):
    return all(new in text for _,_,new in (EDITS if edits is None else edits))

REL=re.compile(r'''((?:\bfrom|\bimport|\bimport\s*\()\s*)(["'])(\.{1,2}/[^"']+)\2''')

def absolutize(text,source_dir,override=None):
    def sub(match):
        target=match.group(3)
        resolved=override.get(target) if override and target in override else os.path.normpath(os.path.join(source_dir,target))
        return f'{match.group(1)}{match.group(2)}{resolved}{match.group(2)}'
    return REL.sub(sub,text)
