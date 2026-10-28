import {agentSessionRegistryRootV1,agentSessionStoreV1,adoptAgentSessionV1,collectAgentSessionsIfDueV1,reclaimAgentSessionSandboxesV1,type AgentSessionReclaimV1,type AgentSessionV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts";

/** 🧹️ Releases what a gone session left on this host: its test sandboxes (images detached first) and its temp root; a live sandbox keeps the root. */
export const agentSessionReclaimV1=(options:Readonly<{parent?:string;alive?:(pid:number)=>boolean}>={}):AgentSessionReclaimV1=>session=>reclaimAgentSessionSandboxesV1(session,{...options.parent===undefined?{}:{parent:options.parent},...options.alive===undefined?{}:{alive:options.alive}});

/** 🤝️ Attaches the process to its agent session at the door of a command: resolves or opens the session, puts its address into the environment so every child inherits it, and collects gone sessions at most once per interval. A session that cannot be attached never fails the command; the failure is reported once. */
export function attachAgentSessionV1(root:string,environment:NodeJS.ProcessEnv=process.env,write:(line:string)=>void=line=>console.warn(line)):AgentSessionV1|undefined{
 try{
  const session=adoptAgentSessionV1(environment,{worktree:root});
  collectAgentSessionsIfDueV1(agentSessionStoreV1(agentSessionRegistryRootV1(environment),session.worktree),agentSessionReclaimV1());
  return session;
 }catch(error){write(`[session] agent session not attached: ${(error as Error).message}`);return undefined;}
}
