export function scanImportsV1(source:string):{static:string[];dynamic:string[]}{
 const text=source.replace(/^#!.*/,"").replace(/\/\*[\s\S]*?\*\//g,m=>m.replace(/[^\n]/g," ")),stat:string[]=[],dyn:string[]=[];
 for(const m of text.matchAll(/(?:^|[;{}\n])\s*(import|export)\s+(type\s+)?([^'"`;]*?\s*from\s*|)["']([^"']+)["']/g)){
  if(m[2])continue;if(m[1]==="export"&&!/from\s*$/.test(m[3]!))continue;
  if(m[1]==="import"&&/^\s*\{[^}]*\}\s*from\s*$/.test(m[3]!)&&!/[A-Za-z_$][\w$]*\s*(,|\})/.test(m[3]!.replace(/type\s+\w+(\s+as\s+\w+)?/g,"")))continue;
  stat.push(m[4]!);}
 for(const m of text.matchAll(/(?<![\w$.])import\s*\(\s*["']([^"']+)["']\s*\)/g))dyn.push(m[1]!);
 return {static:[...new Set(stat)],dynamic:[...new Set(dyn)]};
}
