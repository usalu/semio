import glob,hashlib,json,os,re,sys
CRAB='\U0001F980️';TS='\U0001F7E6️'
def anchors():
    imp='import {bazelCargoClassV1,bazelCargoToolchainDigestV1,BAZEL_CARGO_RULE_V1} from "./%scargo/%s.ts";\n'%(CRAB,TS)
    return [
     ('import',('import {bazelBuildEventsFileV1,',imp+'import {bazelBuildEventsFileV1,')),
     ('action-interface',('restore:string[];cache:BazelCacheDecisionV1}','restore:string[];cache:BazelCacheDecisionV1;exclusive?:true}')),
     ('action-emit',('cache:bazelCacheDecisionV1(task,skipCache)});','cache:bazelCacheDecisionV1(task,skipCache),...(kind==="task"&&bazelCargoClassV1(task)?{exclusive:true as const}:{})});')),
     ('build-render',('attributes.push(["cache",action.cache.cache?"True":"False"]);','if(action.exclusive)attributes.push(["exclusive","True"]);attributes.push(["cache",action.cache.cache?"True":"False"]);')),
     ('rule-resource-set',('execution_requirements = _requirements(ctx), use_default_shell_env = True)','execution_requirements = _requirements(ctx), resource_set = cargo_resources if ctx.attr.exclusive else None, use_default_shell_env = True)')),
     ('rule-attribute',('"uncached_reason": attr.string(), "host_state": attr.string()})\n\ndef _noop_impl','"uncached_reason": attr.string(), "host_state": attr.string(), "exclusive": attr.bool(default = False)})\n\ndef _noop_impl')),
     ('rule-text',('"tasks.bzl"),STARLARK_RULES)','"tasks.bzl"),STARLARK_RULES+"\\n"+BAZEL_CARGO_RULE_V1)')),
     ('fingerprint',('digest.update(implementation);','digest.update(implementation);if(bazelCargoClassV1(task)){const key="\\0toolchain";let known=memo.get(key);if(!known){known=bazelCargoToolchainDigestV1(root,original);memo.set(key,known);}digest.update(await known);}')),
    ]
def apply(text,names=None):
    for name,(old,new) in anchors():
        if new in text: raise SystemExit('already applied: '+name)
        n=text.count(old)
        if n!=1: raise SystemExit('anchor %s occurs %d times'%(name,n))
        text=text.replace(old,new)
    return text
