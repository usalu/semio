import{readFileSync,writeFileSync}from'node:fs';import{join,dirname}from'node:path';
const t=dirname(import.meta.path),s=readFileSync(join(t,'bootstrap-host-frontier.ts'),'utf8');
const head=s.slice(0,s.indexOf("edit('🎭️guest-runtime"));let tail=s.slice(s.indexOf("edit('🔁️poll/🦀️.rs'"));
tail=tail.replace("s=rep(s,'if self.phase==Phase::Poll{152}else{144}', 'if self.phase==Phase::Poll{152}else if matches!(self.phase,Phase::Boot|Phase::CloseContext){200}else{144}');","s=s.replaceAll('if self.phase==Phase::Poll{152}else{144}', 'if self.phase==Phase::Poll{152}else if matches!(self.phase,Phase::Boot|Phase::CloseContext){200}else{144}');");
writeFileSync(join(t,'bootstrap-poll-custody.ts'),head+tail);
