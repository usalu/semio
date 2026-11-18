 const idOfRow=(row,logId,log)=>{
  const logged=logId!==undefined&&(refLog===undefined||refLog===log);
  let id=logged?undefined:byObject.get(row);
  if(id!==undefined)return id;
  const rank=oldRows?rankOf(row.path):-1;
  if(rank>=0){const{first,list}=pathsOf();for(let slot=first[rank];slot<first[rank+1];slot++){const candidate=list[slot];if(D.kinds[D.rowKind[candidate]]===row.kind&&sameFactV1(view.rows[candidate].fact,row.fact)){id=candidate;break;}}}
  if(id===undefined){
   const key=rowKeyV1(row);id=added.get(key);
   if(id===undefined&&rank>=0){const{first,list}=pathsOf();for(let slot=first[rank];slot<first[rank+1];slot++){const candidate=list[slot];if(D.kinds[D.rowKind[candidate]]===row.kind&&rowKeyV1(view.rows[candidate])===key){id=candidate;break;}}}
   if(id===undefined){
    const at=fresh.length;id=oldRows+at;
    if(logged){refLog=log;if(at>=refs.length){const next=new Uint32Array(refs.length*2);next.set(refs);refs=next;}refs[at]=logId;fresh.push(undefined);}else fresh.push(row);
    added.set(key,id);
   }
  }
  if(!logged)byObject.set(row,id);return id;
 };
 let windowFrom=0,windowRows=[],windows=0;
 const freshRow=index=>{
  const row=fresh[index];if(row!==undefined)return row;
  if(index<windowFrom||index>=windowFrom+windowRows.length){
   const end=Math.min(fresh.length,index+FRESH_WINDOW),wanted=[],at=[];
   for(let item=index;item<end;item++)if(fresh[item]===undefined){wanted.push(refs[item]);at.push(item-index);}
   const read=refLog.read(Uint32Array.from(wanted));windowRows=new Array(end-index);
   for(let item=0;item<at.length;item++)windowRows[at[item]]=read[item];
   windowFrom=index;if(++windows%16===0)runtimeCollectGarbageV1(true);
  }
  return windowRows[index-windowFrom];
 };
