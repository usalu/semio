import sys;sys.path.insert(0,__import__("os").path.dirname(__file__))
from lib import edit
W="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/"
edit(W+"🟦️.ts",[
("async function controlled<T>(plan:CargoPhysicalPlanV1<T>,control:CargoWorkspaceControlV1,observations?:CargoPhysicalObservationsV1):Promise<T> {","async function controlled<T>(plan:CargoPhysicalPlanV1<T>,control:CargoWorkspaceControlV1,observations?:CargoPhysicalObservationsV1,beforeRecipe?:()=>void):Promise<T> {"),
('else if(operation.operation==="recipe"){if(!("childBudget" in control)','else if(operation.operation==="recipe"){beforeRecipe?.();if(!("childBudget" in control)'),
("names:readonly string[]=[],observationDirectory?:string,observations?:CargoPhysicalObservationsV1):Promise<CargoPreparationResultV1> {requireIssuedResourceControlV1(control.leaseControl);","names:readonly string[]=[],observationDirectory?:string,observations?:CargoPhysicalObservationsV1,beforeRecipe?:()=>void):Promise<CargoPreparationResultV1> {requireIssuedResourceControlV1(control.leaseControl);"),
("return controlled(preparationPlan(root,selected,names,observationDirectory),control,observations);}","return controlled(preparationPlan(root,selected,names,observationDirectory),control,observations,beforeRecipe);}"),
])
edit(W+"🛠️preparation/🟦️.ts",[
("const prepared=await prepareCargoOwnersControlledV1(root,scope,preparation,packages,directory,observations),scopes=","const prepared=await prepareCargoOwnersControlledV1(root,scope,preparation,packages,directory,observations,mode===\"shared\"?()=>{throw new CargoPreparationExclusiveV1();}:undefined),scopes="),
])
