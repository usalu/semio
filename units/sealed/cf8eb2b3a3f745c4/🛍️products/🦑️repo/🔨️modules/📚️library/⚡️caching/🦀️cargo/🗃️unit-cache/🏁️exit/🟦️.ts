/** 🚧️ The exit status of a process whose work never settled: it is set before the work starts and replaced by the real status, so a process the runtime ends early (an awaited promise that nothing keeps alive, such as a failed request of the fleet cache) fails the compilation instead of reporting a success that produced no output, which lets Cargo start the dependants of a unit that was never built (`extern location for <crate> does not exist`, `could not execute process <build script> (never executed)`).
 * @see https://nodejs.org/api/process.html#processexitcode */
export const UNFINISHED_EXIT_STATUS_V1=70;

/** 🏁️ Runs `work` as the body of a process and ends it with the status `work` answers; a rejection ends it with status 1 and its message, work that never settles ends it with {@link UNFINISHED_EXIT_STATUS_V1}. Every process Cargo executes for a unit (the compiler stand-in, the build script stand-in) ends this way and no other. */
export function exitWithStatusOfV1(work:()=>Promise<number>):void{
 process.exitCode=UNFINISHED_EXIT_STATUS_V1;
 const finish=(status:number)=>{process.exitCode=status;setImmediate(()=>process.exit(status));};
 work().then(finish,error=>{console.error(String((error as Error)?.message??error));finish(1);});
}
