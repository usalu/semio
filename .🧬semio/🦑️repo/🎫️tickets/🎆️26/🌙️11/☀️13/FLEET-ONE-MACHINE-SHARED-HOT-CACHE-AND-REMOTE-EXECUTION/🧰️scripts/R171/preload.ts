import{plugin}from"bun";
import{readFileSync}from"node:fs";
const live=process.env.R171_LIVE!,staged=process.env.R171_STAGED!;
plugin({name:"r171-stage",setup(build){build.onLoad({filter:/📥️load\/🟦️\.ts$/},args=>args.path===live?{contents:readFileSync(staged,"utf8"),loader:"ts"}:undefined);}});
