import{test}from"bun:test";
import{spawn}from"node:child_process";
import{openSync}from"node:fs";
test("leaks a detached process and 3000 files",()=>{
 spawn("sleep",["60"],{detached:true,stdio:"ignore"}).unref();
 for(let index=0;index<3000;index++)openSync("/dev/null","r");
});
