  for(const row of registration.resumed??[]){
   const dot=row.task.lastIndexOf("."),oldId=row.task.slice(0,dot),attempt=Number(row.task.slice(dot+1)),key=`${row.instance}|${row.action.hash}`,existing=this.byKey.get(key);
   if(existing?.stage==="EXECUTING"||this.operations.has(oldId))continue;
   if(existing){
    const index=this.queue.indexOf(existing);if(index>=0)this.queue.splice(index,1);
    this.operations.delete(existing.id);existing.id=oldId;existing.attempt=attempt;existing.stage="EXECUTING";existing.worker=id;existing.startedAt=this.clock();if(existing.platform.length===0)existing.adopted=true;
    this.operations.set(oldId,existing);this.workers.get(id)!.running.add(row.task);this.touch(existing);
    this.emit("assigned",`${this.label(existing)} adopted from ${registration.name}`);continue;
   }
   const operation:Operation={id:oldId,instance:row.instance,action:row.action,key,platform:[],timeoutSeconds:this.options.defaultActionSeconds,stage:"EXECUTING",version:0,createdAt:this.clock(),queuedAt:this.clock(),attempt,failures:0,tried:new Set(),offload:false,subscribers:new Set(),cancelRequested:false,adopted:true,worker:id,startedAt:this.clock()};
   this.operations.set(oldId,operation);this.byKey.set(key,operation);this.workers.get(id)!.running.add(row.task);
   operation.lingerTimer=setTimeout(()=>{this.cancel(operation);},this.options.lingerSeconds*1000);operation.lingerTimer.unref();
   this.emit("submitted",`${this.label(operation)} adopted from ${registration.name}`);
  }
