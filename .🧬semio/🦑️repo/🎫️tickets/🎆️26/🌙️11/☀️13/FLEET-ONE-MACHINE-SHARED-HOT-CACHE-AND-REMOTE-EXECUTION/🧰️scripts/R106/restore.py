import json,shutil,os
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R106"
for path,backup in json.load(open(SC+"/backups.json")).items():
    shutil.copy2(backup,path);print("restored",os.path.basename(path))
