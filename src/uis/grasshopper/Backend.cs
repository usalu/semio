using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;
using static Grpc.Gateway.ProtocGenOpenapiv2.Options.SecurityScheme.Types;

namespace Semio.UI.Grasshopper
{
    public static class Backend
    {
        private static Process _singleton = start();

        private static Process start()
        {
            if (!_singleton.HasExited)
                _singleton.Kill();

            ProcessStartInfo startInfo = new ProcessStartInfo();
            startInfo.CreateNoWindow = false;
            startInfo.UseShellExecute = false;
            startInfo.WorkingDirectory =
                Path.Combine(Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location), "semio");
            startInfo.FileName = "semio.exe";
            startInfo.WindowStyle = ProcessWindowStyle.Hidden;

            return Process.Start(startInfo);
        }

        public static void clearCache()
        {
            _singleton = start();
        }

    }
}
