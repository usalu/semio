import sys
R="/Users/ueli/Documents/semio"
O=R+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/dbg2-d/orig"
def F(n):return R+"/"+[l.split(" ",1)[1].strip() for l in open(O+"/map.txt",encoding="utf8") if l.startswith(str(n)+" ")][0]
def rep(path,old,new,count=1):
    t=open(path,encoding="utf8").read();n=t.count(old)
    if n==0:print("SKIP",old[:60]);return
    if n!=count:print("FAIL",n,count,old[:80]);sys.exit(1)
    open(path,"w",encoding="utf8").write(t.replace(old,new));print("ok",old[:50])
f=F(56);rep(f,"let (order, work) = execute(&keys);","let (order, _) = execute(&keys);")
f=F(27)
rep(f,"let mut closed=close_builder(&mut builder,&mut total);","close_builder(&mut builder,&mut total);")
rep(f,"if let Some(root)=output{closed+=close_value(root,&mut total);}","if let Some(root)=output{close_value(root,&mut total);}")
f=F(90)
rep(f,"let mut turns=0;let mut close_turns=0;","let mut turns=0;")
rep(f,"total.1+=heap.1;close_turns+=1;close_work","total.1+=heap.1;close_work")
rep(f,"let(born,released,closed)=close(value);total.0+=born;total.1+=released;close_turns+=closed;","let(born,released,_)=close(value);total.0+=born;total.1+=released;",2)
rep(f,"let(born,released,closed)=close(original);total.0+=born;total.1+=released;close_turns+=closed;","let(born,released,_)=close(original);total.0+=born;total.1+=released;")
f=F(205)
rep(f,"for(command_index,command)in commands().into_iter().enumerate(){","for command in commands(){",2)
rep(f,"let mut output=None;let mut turns=0;","let mut output=None;")
rep(f,"(progress.retained_capacity_bytes,progress.released_bytes));turns+=1;if output.is_some(){break}}","(progress.retained_capacity_bytes,progress.released_bytes));if output.is_some(){break}}")
