using System;
using System.CodeDom.Compiler;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using SemIO;
using SemIO.Parsing;
using SemIO.Parsing.ParserModels.Project;

namespace SemIOCompiler
{
    class Program
    {
        public const int WaitingTimeFast = 500;
        public const int WaitingTime = 1500;
        static void Main(string[] args)
        {
            /*string path = @"D:\Dokumente\GitHub\SemIO\src\SemIO_VisualStudio2019\SemIOConsoleApp\bin\Debug\SemIOCode\";
            string fileName = "SampleCode.sio";
            string semIOCode = System.IO.File.ReadAllText(path+ fileName);
            string outputPath = @"E:\Dokumente\GitHub\SemIO\src\SemIO_VisualStudio2019\SemIOConsoleApp\bin\Debug\";

            CompilerResults compilerResults;
            bool isCompiled;
            try
            {
                compilerResults = Compiler.CompileProjectAssembly("SemIOTestAssembly", semIOCode,outputPath);
                isCompiled = true;
            }
            catch (Exception)
            {
                isCompiled = false;
            }
            Console.WriteLine("Compilation successful? " + isCompiled);
            Console.ReadLine();*/

            /*string print = SemIORegexs.abstractionLevelPattern;
            Console.WriteLine(print);
            Console.ReadLine();*/
            //System.IO.File.WriteAllText(path+"Pattern.txt", print);

            StartSemIO();

        }

        private static void StartSemIO()
        {
            //Console.WriteLine("Hello there!");
            //Thread.Sleep(WaitingTime);
            //Console.WriteLine("\nI am your semIO compiler.");
            //Thread.Sleep(WaitingTime);
            //Console.WriteLine("\nNice to meet you!");
            //Thread.Sleep(WaitingTime);

            //Compile(GetName(), GetSemIOCode(true), GetOutputPath(false,true),true);

            string outputPath = @"E:\Dokumente\GitHub\SemIO\src\SemIO_VisualStudio2019\SemIOConsoleApp\bin\Debug\";

            CompileExampleProject(outputPath);
        }

        private static void CompileExampleProject(string outputPath)
        {
            CompilerResults compilerResults;
            ExampleProject exampleProject = new ExampleProject();
            compilerResults = Compiler.CompileProjectAssembly("ExampleProject", exampleProject.projectModel, outputPath);
            PrintCompilerResult(compilerResults);
            string assemblyName = compilerResults.CompiledAssembly.GetName().Name;
            Console.WriteLine("\nThe assembly you created is stored under:");
            Console.WriteLine(outputPath + @"*\CompilerResults\" + assemblyName + ".dll");
        }

        private static string GetName()
        {
            Console.WriteLine("\nHow do you wish to call your new project?");
            return Console.ReadLine();
        }
        private static string GetOutputPath(bool somewhereElse, bool fristTime)
        {
            if (!somewhereElse)
            {
                Console.WriteLine("\nDo you want to store your dynamic linked library (DLL) here?" +
                                  @"A new folder *\CompilerResults will be created.");
                string answer = Console.ReadLine();
                if (answer == "sure" || answer == "Sure" || answer == "ofc"
                    || answer == "Yes" || answer == "yes" || answer == "y"
                    || answer == "")
                {
                    return "";
                }
                
            }

            if (fristTime)
                Console.WriteLine("\nIn which path do you want store your dynamic linked library (DLL)?");
            else
                Console.WriteLine("\nCan you give it again to me?");

            Thread.Sleep(WaitingTime);
            string pathOutput = Console.ReadLine();
            if (!pathOutput.EndsWith(@"\"))
                pathOutput = pathOutput + @"\";
            if(Directory.Exists(pathOutput))
                return pathOutput;
            Console.WriteLine("\nSorry, I couldn't find that directory.");
            Thread.Sleep(WaitingTime);
            return GetOutputPath(true,false);
        }

        private static string GetSemIOCode(bool firstTime)
        {
            if (firstTime)
                Console.WriteLine("\nHow is your file called? " + @" A semIO file has the *.sio extension and needs to be in subfolder *\SemIOCompiler\SemIOCode.");
            else
                Console.WriteLine("\nCan you give me the file name another time?" + "\nDon't forget the file needs a *.sio extension " + 
                                  @" and needs to be in subfolder *\SemIOCompiler\SemIOCode.");


            string semIOFileName = Directory.GetCurrentDirectory() + @"\SemIOCode\" + Console.ReadLine();
            try
            {
                string semIOCode = System.IO.File.ReadAllText(semIOFileName.EndsWith(".sio") ? semIOFileName : semIOFileName + ".sio");
                Console.WriteLine("\nFound it!");
                Thread.Sleep(WaitingTime);
                return semIOCode;
            }
            catch (Exception)
            {
                Console.WriteLine("\nSomething went wrong.");
                Thread.Sleep(WaitingTime);
                return GetSemIOCode(false);
            }
        }

        private static void Compile(string name, string semIOCode, string path, bool firstTime)
        {
            if (firstTime)
            {
                Console.WriteLine("\nLet's start to compile!");
                Thread.Sleep(WaitingTime);
            }
            else
            {
                Console.WriteLine("\nLet's try again.");
                Thread.Sleep(WaitingTime);
            }
            
            CompilerResults compilerResults;
            bool isCompiled;
            try
            {

                compilerResults = Compiler.CompileProjectAssembly(name, new ProjectModel(name,"",semIOCode),path);

                Console.WriteLine("\nThat's it...");
                Thread.Sleep(WaitingTime);
                Console.WriteLine("\nYou made it!");
                Thread.Sleep(WaitingTime);

                string assemblyName = compilerResults.CompiledAssembly.GetName().Name;

                PrintCompilerResult(compilerResults);

                Console.WriteLine("\nThe assembly you created is stored under:");
                Console.WriteLine(path + @"*\CompilerResults\" + assemblyName + ".dll");
                Thread.Sleep(WaitingTime);
                Console.WriteLine("\nThat was fun!");
                Thread.Sleep(WaitingTime);
                Console.WriteLine("\nDo you wish to compile another project?");
                string answer1 = Console.ReadLine();
                if (answer1 == "sure" || answer1 == "Sure" || answer1 == "maybe"
                    || answer1 == "Yes" || answer1 == "yes" || answer1 == "y" || answer1 == "")
                    Compile(GetName(), GetSemIOCode(false), GetOutputPath(false, true), false);
                else
                {
                    EndQuestion();
                }
            }
            catch (Exception)
            {
                Console.WriteLine("\nUps, there must be a syntax error in your file.");
                Thread.Sleep(WaitingTime);
                Console.WriteLine("\nShould I try again with the same file?");
                Thread.Sleep(WaitingTime);
                string answer = Console.ReadLine();
                if (answer == "Yes" || answer == "yes" || answer == "y" || answer == "")
                    Compile(name, semIOCode, path, false);
                Compile(name, GetSemIOCode(false), path, false);
            }
        }
        private static void EndQuestion()
        {
            Console.WriteLine("\nSee you maybe later?");
            string answer = Console.ReadLine();
            if (answer == "Ciao" || answer == "ciao" || answer == "bye"
                || answer == "Bye" || answer == "Bye bye" || answer == "Sure"
                || answer == "sure" || answer == "Maybe" || answer == "maybe"
                || answer == "Yes" || answer == "yes" || answer == "y"
                || answer == "Goodbye" || answer == "goodbye"
                || answer == "Good bye" || answer == "good bye")
                return;
            if (answer == "")
            {
                Console.WriteLine("\nAren't you going to say goodbye?");
                Thread.Sleep(WaitingTime);
                Console.WriteLine("\nLet's try it again!");
                Thread.Sleep(WaitingTime);
                EndQuestion();
            }
            Console.WriteLine("\nThat is not what I want to hear!");
            Thread.Sleep(WaitingTime);
            EndQuestion();
        }

        public static void PrintCompilerResult(CompilerResults compilerResults)
        {

            Assembly assembly = compilerResults.CompiledAssembly;
            string assemblyName = assembly.GetName().Name;
            Type[] types = assembly.GetTypes();
            Type[] abstractionLevelTypes = types.Where(x => x.Namespace.EndsWith("Repository")).ToArray();
            Type[] parameterTypeTypes = types.Where(x => x.Namespace.EndsWith("ParameterTypes")).ToArray();
            Type[] thingTypes = types.Where(x => x.Namespace.EndsWith("ThingTypes")).ToArray();
            Console.WriteLine("\n" + assemblyName + $" contains in total " +
                              HumanFriendlyInteger.IntegerToWritten(abstractionLevelTypes.Length).ToLower()
                              + " abstraction levels.");
            Thread.Sleep(WaitingTime);

            Console.WriteLine("\nHere some statistics: ");
            Thread.Sleep(WaitingTime);
            foreach (var al in abstractionLevelTypes)
            {
                var thingsAL = thingTypes.Where(x => x.Namespace.StartsWith(
                    "AbstractionLevels." + al.Name + "Repository.ThingTypes")).ToArray();
                var parameterTypesAL = parameterTypeTypes.Where(x => x.Namespace.StartsWith(
                    "AbstractionLevels." + al.Name + "Repository.ParameterTypes")).ToArray();

                Console.WriteLine("\nThe abstraction level " + al.Name + $" has " +
                                  HumanFriendlyInteger.IntegerToWritten(thingsAL.Length).ToLower()
                                  + " things and " +
                                  HumanFriendlyInteger.IntegerToWritten(parameterTypesAL.Length).ToLower()
                                  + " parameter types.");
                Thread.Sleep(WaitingTime);
                Console.WriteLine("\nThose are your things: ");
                Thread.Sleep(WaitingTime);
                foreach (var thingType in thingsAL)
                {
                    var parameters = thingType.GetProperties();
                    Console.WriteLine("\n" + thingType.Name + " has following properties: ");
                    Thread.Sleep(WaitingTime);
                    foreach (var prm in parameters)
                    {
                        Console.WriteLine("\t" + prm.Name + " is of type " + prm.PropertyType.Name);
                        Thread.Sleep(WaitingTimeFast);
                    }
                }
                Console.WriteLine("\nThose are your parameter types: ");
                Thread.Sleep(WaitingTime);
                foreach (var prmType in parameterTypesAL)
                {
                    var prmValues = prmType.GetEnumNames();
                    bool isMultiset = prmType.CustomAttributes.ToList()
                        .Exists(x => x.AttributeType == typeof(FlagsAttribute));
                    Console.WriteLine("\n" + prmType.Name + " has following values " +
                                      "(Multiple values " + (isMultiset ? "can" : "can't") + " be selected): ");
                    Thread.Sleep(WaitingTime);
                    foreach (var prm in prmValues)
                    {
                        Console.WriteLine("\t" + prm);
                        Thread.Sleep(WaitingTimeFast);
                    }
                }

            }
        }
    }

    public static class HumanFriendlyInteger
    {
        static string[] ones = new string[] { "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine" };
        static string[] teens = new string[] { "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen" };
        static string[] tens = new string[] { "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety" };
        static string[] thousandsGroups = { "", " Thousand", " Million", " Billion" };

        private static string FriendlyInteger(int n, string leftDigits, int thousands)
        {
            if (n == 0)
            {
                return leftDigits;
            }

            string friendlyInt = leftDigits;

            if (friendlyInt.Length > 0)
            {
                friendlyInt += " ";
            }

            if (n < 10)
            {
                friendlyInt += ones[n];
            }
            else if (n < 20)
            {
                friendlyInt += teens[n - 10];
            }
            else if (n < 100)
            {
                friendlyInt += FriendlyInteger(n % 10, tens[n / 10 - 2], 0);
            }
            else if (n < 1000)
            {
                friendlyInt += FriendlyInteger(n % 100, (ones[n / 100] + " Hundred"), 0);
            }
            else
            {
                friendlyInt += FriendlyInteger(n % 1000, FriendlyInteger(n / 1000, "", thousands + 1), 0);
                if (n % 1000 == 0)
                {
                    return friendlyInt;
                }
            }

            return friendlyInt + thousandsGroups[thousands];
        }

        public static string IntegerToWritten(int n)
        {
            if (n == 0)
            {
                return "Zero";
            }
            else if (n < 0)
            {
                return "Negative " + IntegerToWritten(-n);
            }

            return FriendlyInteger(n, "", 0);
        }
    }
}
