@echo off
node "%~dp0repo-shim.mjs" %*
exit /b %ERRORLEVEL%
