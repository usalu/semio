/** 🔤️ The text of one fact inside the identity of an observation row. */
export function factKeyV1(fact:unknown):string;
/** #️⃣ A 32 bit FNV-1a hash of a text, continued from `seed`. */
export function hashTextV1(text:string,seed:number):number;
/** #️⃣ The 53 bit hash of one row identity. */
export function rowHashV1(kind:string,path:string,factKey:string):number;
/** 🔢️ The table of canonical observation rows. */
export type RowTableV1<Row extends{kind:string;path:string;fact:unknown}>=Readonly<{canonical(kind:string,path:string,fact:unknown,make:()=>Row):Row;id(row:Row):number;size():number;clear():void}>;
/** 🔢️ Creates the table of canonical observation rows: equal (kind, path, fact) is one row and one dense number until `clear()`. */
export function createRowTableV1<Row extends{kind:string;path:string;fact:unknown}=any>(options?:{factKey?:(fact:unknown)=>string;hash?:(kind:string,path:string,factKey:string)=>number}):RowTableV1<Row>;
