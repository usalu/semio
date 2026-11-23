import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored command offload policy.
 * @see ../🧬️schema/🔣️.json */
export const COMMAND_POLICY_V1=schema.policy;

/** 🧭️ Where a tool of the allow-list comes from: `resolved` (the worker finds it itself and offers it by the digest of its own file) or `shipped` (the requester sends it as an input and the worker checks its digest). */
export type CommandToolSourceV1="resolved"|"shipped";

/** 🧰️ The names of the tool allow-list. */
export const commandToolNamesV1=():readonly string[]=>Object.keys(COMMAND_POLICY_V1.tools);

/** 🧭️ The source of an allow-listed tool, `undefined` for a name that is not on the list. */
export function commandToolSourceV1(name:string):CommandToolSourceV1|undefined{
 const row=(COMMAND_POLICY_V1.tools as Readonly<Record<string,{source:CommandToolSourceV1}>>)[name];
 return Object.hasOwn(COMMAND_POLICY_V1.tools,name)?row?.source:undefined;
}

/** 💾️ The megabytes the placement reserves for one run of an allow-listed tool (its predicted peak), 0 for none. */
export const commandToolPeakMegabytesV1=(name:string):number=>Object.hasOwn(COMMAND_POLICY_V1.tools,name)?(COMMAND_POLICY_V1.tools as Readonly<Record<string,{peakMegabytes:number}>>)[name]!.peakMegabytes:0;

/** 🏷️ The program of a command: `@command:<tool>/<sha-256 of the tool file>`, so the digest of the tool is part of the action and an equal command with another tool binary is another action. */
export const commandProgramV1=(name:string,digest:string):string=>`${COMMAND_POLICY_V1.program.placeholder}:${name}${COMMAND_POLICY_V1.program.separator}${digest}`;

/** 🔎️ Reads a command program: its tool name and tool digest, `undefined` for every other program. */
export function parseCommandProgramV1(program:string):Readonly<{name:string;digest:string}>|undefined{
 const match=new RegExp(`^${COMMAND_POLICY_V1.program.placeholder}:([a-z][a-z0-9-]{0,63})${COMMAND_POLICY_V1.program.separator}([0-9a-f]{64})$`,"u").exec(program);
 return match===null?undefined:{name:match[1]!,digest:match[2]!};
}

/** 🏷️ The key of a tool in a worker's table and the value of its platform property: `<tool>/<digest>`. */
export const commandToolKeyV1=(name:string,digest:string):string=>`${name}/${digest}`;

/** 🏷️ The platform property a worker offers for one of its tools: `semio-command-<tool>` with the tool's digest as the value, so the scheduler places a command only on a machine that has exactly that tool. */
export const commandToolPropertyV1=(name:string,digest:string):Readonly<{name:string;value:string}>=>({name:COMMAND_POLICY_V1.property.tool+name,value:digest});

/** 💬️ Renders one catalogue text in one language. */
export function commandTextV1(language:"en"|"de",key:string,variables:Readonly<Record<string,string|number>>={}):string{
 const message=(schema.messages[language] as Readonly<Record<string,string>>)[key];
 if(message===undefined)throw Error(`Declared command offload text required: ${key}`);
 return message.replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}

/** 🌍️ One text in every language, one line per language. */
export const commandTextsV1=(key:string,variables:Readonly<Record<string,string|number>>={}):string=>(["en","de"] as const).map(language=>commandTextV1(language,key,variables)).join("\n");
