import { mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const output=resolve(import.meta.dir,"../🗑️generated/compiler-probe");mkdirSync(output,{recursive:true});
const source=join(output,"📜️script.ts"), binary=join(output,"compiler-probe.exe");
writeFileSync(source, String.raw`using System;
using System.IO;
using System.Linq;
using System.Text;
using System.Diagnostics;
class Probe {
  static string Quote(string value) {
    var b=new StringBuilder("\"");int slash=0;
    foreach(char c in value){if(c=='\\'){slash++;continue;}b.Append('\\',c=='"'?slash*2+1:slash);b.Append(c);slash=0;}
    b.Append('\\',slash*2);b.Append('"');return b.ToString();
  }
  static int Main(string[] args) {
    var start=new ProcessStartInfo(args[0],string.Join(" ",args.Skip(1).Select(Quote)));start.UseShellExecute=false;
    string original=Environment.GetEnvironmentVariable("PATH")??"", build=Environment.GetEnvironmentVariable("CARGO_BUILD_BUILD_DIR")??"";
    var rows=original.Split(';');int omitted=0;
    if(Environment.GetEnvironmentVariable("SEMIO_TEST_DLL_PATH_FILTER")=="1" && build.Length>0) {
      string root=Path.GetFullPath(build).TrimEnd('\\')+"\\";
      start.EnvironmentVariables["PATH"]=string.Join(";",rows.Where(path=>{
        try{if(Path.GetFullPath(path).StartsWith(root,StringComparison.OrdinalIgnoreCase)&&Directory.Exists(path)&&!Directory.EnumerateFiles(path,"*.dll").Any()){omitted++;return false;}}catch{}return true;
      }));
    }
    Console.Error.WriteLine("[DEBUG] compiler path chars="+original.Length+" dirs="+rows.Length+" omitted="+omitted+" finalChars="+start.EnvironmentVariables["PATH"].Length);
    using(var child=Process.Start(start)){child.WaitForExit();return child.ExitCode;}
  }
}`);
const result=spawnSync("C:/Windows/Microsoft.NET/Framework64/v4.0.30319/csc.exe",["/nologo","/target:exe","/platform:x64","/out:"+binary,source],{stdio:"inherit",windowsHide:true});
if(result.status!==0)throw Error("Compiler probe build failed");console.log(binary);
