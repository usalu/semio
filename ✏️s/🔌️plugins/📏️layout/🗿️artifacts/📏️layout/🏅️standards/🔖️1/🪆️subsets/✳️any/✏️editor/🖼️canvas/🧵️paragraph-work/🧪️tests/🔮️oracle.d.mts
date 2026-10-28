/** 🔮️ Test-only oracle exposes scalar observations without exporting third-party tools or types. */
export function run(root:string):Promise<{styles:number;transforms:number;geometryCases:number;pixels:number;cancellations:number;refusals:number;maxPixelError:number;oracle:string}>;
export function repositoryRoot():string;
