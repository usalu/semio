Option Explicit
Dim fs, file, line, shell, result
Set fs = CreateObject("Scripting.FileSystemObject")
Set file = fs.OpenTextFile(WScript.Arguments(0), 1, False, -1)
line = file.ReadAll
file.Close
Set shell = CreateObject("WScript.Shell")
result = shell.Run(line, 0, True)
WScript.Quit result
