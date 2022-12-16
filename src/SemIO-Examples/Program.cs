using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using SemIO;

namespace SemIO
{
    static class Program
    {
        static void Main(string[] args)
        {
            var dietzoldwerk = new Dietzoldwerk();
            Compiler.CompileProjectAssembly("Dietzoldwerk", dietzoldwerk.projectModel);
            Console.WriteLine("Success!");
            Console.ReadKey();

        }
    }
}
