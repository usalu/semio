Option Explicit
Dim fs, folder, input, trace, file, line, shell, result
Set fs = CreateObject("Scripting.FileSystemObject")
For Each folder In fs.GetFolder(fs.GetParentFolderName(WScript.ScriptFullName)).SubFolders
    If Right(folder.Name, 9) = "generated" Then input = fs.BuildPath(folder.Path, WScript.Arguments(0) & ".txt")
Next
Set trace = fs.CreateTextFile(input & ".debug", True)
trace.WriteLine "[DEBUG] opened trace"
Set shell = CreateObject("WScript.Shell")
Set file = fs.OpenTextFile(shell.ExpandEnvironmentStrings("%LOCALAPPDATA%") & "\semio\fleet\quiet-tasks\semio-fleet-store-mount.txt", 1, False, -1)
line = file.ReadAll
file.Close
trace.WriteLine "[DEBUG] read local task command"
trace.WriteLine "[DEBUG] running child"
result = shell.Run("cmd.exe /d /c exit 17", 0, True)
trace.WriteLine "[DEBUG] child exited " & result
trace.Close
WScript.Quit result
