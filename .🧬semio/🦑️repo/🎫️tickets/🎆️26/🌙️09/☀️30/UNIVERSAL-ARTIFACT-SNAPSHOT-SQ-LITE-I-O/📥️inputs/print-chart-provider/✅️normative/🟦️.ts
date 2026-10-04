/** 🧭️ Authored persistence shares every normative bound and enum; only language presence differs. */
function normative(shape:Shape,value:ObjectValue):void{
 if(shape.table==="chart_guide"){
  if(value.ticks!==undefined&&(!Number.isInteger(number(value.ticks))||number(value.ticks)<1))invalid("guide ticks must be an integer of at least one");
  if(value.orient!==undefined&&!["top","right","bottom","left"].includes(text(value.orient)))invalid("unknown guide orientation");
 }
 if(shape.table==="chart_scale"&&value.options!==undefined){
  const options=object(value.options);
  for(const name of["nice","constant","ticks"]){const item=options[name];if(item!==undefined&&!(name==="nice"&&typeof item==="boolean")&&number(item)<=0)invalid("positive scale option required");}
  for(const name of["padding","paddingOuter"]){if(options[name]!==undefined&&number(options[name])<0)invalid("nonnegative scale padding required");}
  for(const name of["paddingInner","align"]){if(options[name]!==undefined&&(number(options[name])<0||number(options[name])>1))invalid("scale fraction must be between zero and one");}
  if(options.base!==undefined&&(number(options.base)<=0||number(options.base)===1))invalid("scale base must be positive and differ from one");
  if(options.interpolator!==undefined&&!["number","round","rgb","lab","hcl","oklab"].includes(text(options.interpolator)))invalid("unknown scale interpolator");
  if(options.interval!==undefined&&!["auto","day","week","month","year"].includes(text(options.interval)))invalid("unknown scale interval");
  if(options.scheme!==undefined&&text(options.scheme).length===0)invalid("scale scheme must be nonempty");
  if((value.kind==="band"||value.kind==="point")&&Object.hasOwn(options,"unknown"))invalid("band and point scales do not admit unknown");
 }
}
