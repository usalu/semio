
package main

import (
 "context"
 "encoding/json"
 "fmt"
 "io/fs"
 "os"
 "path/filepath"
 "strings"
 "time"
 workspace "github.com/usalu/semio/repo/workspace"
 model "github.com/usalu/semio/repo/model"
 codebase "github.com/usalu/semio/repo/codebase"
)
func main(){
 if len(os.Args)!=2||!filepath.IsAbs(os.Args[1]){panic("Explicit Source census root required")}
 root:=os.Args[1];workspace.SetRootDir(root);ctx,cancel:=context.WithTimeout(context.Background(),30*time.Second);defer cancel();started:=time.Now()
 counts:=map[string]int64{};depths:=map[int]int64{};groups:=map[string]int64{}
 err:=filepath.WalkDir(root,func(path string,entry fs.DirEntry,failure error)error{
  if err:=ctx.Err();err!=nil{return err};if failure!=nil{return failure};counts["visited"]++
  relative,err:=filepath.Rel(root,path);if err!=nil{return err};relative=filepath.ToSlash(relative);if relative=="."{return nil}
  if entry.Type()&os.ModeSymlink!=0{counts["symlinks"]++;return nil}
  if workspace.IsRepoExcludedPath(path){counts["canonicalExcluded"]++;if entry.IsDir(){return filepath.SkipDir};return nil}
  if workspace.IsGitIgnored(path){counts["gitIgnored"]++;if entry.IsDir(){counts["gitIgnoredDirectories"]++;return filepath.SkipDir};return nil}
  if entry.IsDir(){counts["walkFolders"]++;if model.IsGenerated(path)||model.IsGeneratedFolder(relative){counts["generatedFolders"]++}else{counts["eligibleFolders"]++};return nil}
  info,err:=entry.Info();if err!=nil{return err};if !info.Mode().IsRegular(){counts["nonregular"]++;return nil}
  counts["walkFiles"]++;counts["walkBytes"]+=info.Size()
  if model.IsGenerated(path){counts["generatedFiles"]++;return nil}
  counts["eligibleFiles"]++;counts["eligibleBytes"]+=info.Size();depths[strings.Count(relative,"/")]++
  parts:=strings.Split(relative,"/");if len(parts)>2{parts=parts[:2]};groups[strings.Join(parts,"/")]++
  return nil
 });if err!=nil{panic(err)}
 elapsed:=time.Since(started).Milliseconds();technologyStart:=time.Now();technologies:=codebase.LoadTechnologies();bundles:=0;for _,technology:=range technologies{bundles+=len(technology.Bundles)}
 data,err:=json.Marshal(map[string]any{"elapsedMs":elapsed,"counts":counts,"fileDepths":depths,"groups":groups,"technologies":len(technologies),"bundles":bundles,"technologyMs":time.Since(technologyStart).Milliseconds()});if err!=nil{panic(err)};fmt.Println("[DEBUG] native-exact-census "+string(data))
}
