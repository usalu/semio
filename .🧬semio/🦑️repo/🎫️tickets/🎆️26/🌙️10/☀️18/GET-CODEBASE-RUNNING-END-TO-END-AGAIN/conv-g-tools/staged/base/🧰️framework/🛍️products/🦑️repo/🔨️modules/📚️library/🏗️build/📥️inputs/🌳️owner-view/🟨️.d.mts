/** 🌳️ The owner path of a project root. */
export function ownerOfProjectV1(root:string,deliveryScopeNames:readonly string[]):string;
/** 🌳️ The policy row that makes subtrees of an owner private to its own tasks. */
export function ownerViewV1(owner:string,views:readonly {readonly owner:string;readonly private:readonly string[]}[]|undefined):{readonly owner:string;readonly private:readonly string[]}|undefined;
/** 🌳️ The owner globs every dependent sees and the ones only the project's own tasks see. */
export function ownerTreeGlobsV1(owner:string,extensions:string,privateNames:readonly string[],entries:readonly {readonly name:string;readonly directory:boolean}[]):{shared:string[];private:string[]};
