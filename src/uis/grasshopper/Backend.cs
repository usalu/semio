using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.GUI.Script;
using static Grpc.Gateway.ProtocGenOpenapiv2.Options.SecurityScheme.Types;

namespace Semio.UI.Grasshopper
{
    public static class Backend
    {
        private static string semioPath => Path.Combine(Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location), "semio");
        private static string semioExe => Path.Combine(semioPath, "semio.exe");

        private static Process _singleton = start();

        public static Process getSemioProcess()
        {
            var processes = Process.GetProcessesByName("semio");

            if (processes.Length == 1)
                return processes[0];

            var terminalProcesses = Process.GetProcessesByName("WindowsTerminal");

            foreach (var terminalProcess in terminalProcesses)
                if (terminalProcess.MainWindowTitle == semioExe)
                    return terminalProcess;
            return null;
        }

        private static Process restart()
        {
            var processes = Process.GetProcessesByName("semio");

            foreach (var process in processes)
                process.Kill();

            var terminalProcesses = Process.GetProcessesByName("WindowsTerminal");

            foreach (var terminalProcess in terminalProcesses)
                if (terminalProcess.MainWindowTitle == semioExe)
                    terminalProcess.Kill();
            return start();
        }
        public static Process start()
        {
            var existingProcess = getSemioProcess();
            if (existingProcess != null)
                return existingProcess;

            ProcessStartInfo startInfo = new ProcessStartInfo();
            startInfo.CreateNoWindow = false;
            startInfo.UseShellExecute = false;
            startInfo.WorkingDirectory = semioPath;
            startInfo.FileName = semioExe;
            startInfo.WindowStyle = ProcessWindowStyle.Hidden;

            return Process.Start(startInfo);

        }
        public static void clearCache()
        {
            _singleton = restart();
        }

        public static bool IsRunning
        {
            get
            {
                if (_singleton == null)
                    return false;
                return !_singleton.HasExited;
            }
        }
    }
}
