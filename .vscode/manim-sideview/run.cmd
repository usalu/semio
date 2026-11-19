@echo off
REM 🎬 Windows Manim Sideview launcher → run.py (sets SEMIO_TEACHING_* then execs manim).
setlocal
set "HERE=%~dp0"
python "%HERE%run.py" %*
exit /b %ERRORLEVEL%
