using System.Text.RegularExpressions;

namespace SemIO.Parsing
{
    public static class SemIORegexs
    {
        public static string spacePattern = "(" + @"((( ){4})|\t)" + ")";
        //name containing alphanumeric tokens or underscores, starting with a letter or underscore
        public static string namePattern = "(" + @"[A-Za-z_][A-Za-z0-9_äöüÄÖÜ]*" + ")";
        //number that never starts with a 0
        public static string numberPattern = "(" + "([1-9]+[0-9]*)|([0-9])" + ")";
        //mathematical arithmetic operators
        public static string arithmeticOperatorPattern = "(" + @"[-+*/]" + ")";
        //name that is wrapped by all whitespaces around it
        public static string nameWrappedWithSpacesPattern = "(" + @"\s*" + namePattern + @"\s*" + ")";
        //number that is wrapped by all whitespaces around it
        public static string numberWrappedWithSpacesPattern = "(" + @"\s*" + numberPattern + @"\s*" + ")";
        //binds all whitspaces until the next new line
        public static string endOfLinePattern = "(" + @"( |\t)*(\r?\n)" + ")";
        /*//colon that announces the code block
        public static string codeBlockExpectedPattern = "(" + @"\s*:" + endOfLinePattern + ")";*/
        //Non colon syntax style
        public static string codeBlockExpectedPattern = "(" + endOfLinePattern + ")";
        //any description marked by quotation marks
        public static string descriptionPattern = "(" + "\"" + "[^\"]*" + "\"" + endOfLinePattern + "+" + ")";// 
        public static string descriptionOneSpacePattern = "(" + spacePattern + descriptionPattern + ")";
        public static string descriptionTwoSpacePattern = "(" + spacePattern + "{2}" + descriptionPattern + ")";
        //an argument is a name wrapped by round brackets
        public static string argumentPattern = "(" + "[(]" + nameWrappedWithSpacesPattern + "[)]" + ")";
        // arguments that are separated by a comma and wrapped by round brackets
        public static string argumentsPattern = "(" + "[(]" + nameWrappedWithSpacesPattern + "(" + "," + nameWrappedWithSpacesPattern + ")" + "*" + "[)]" + ")";
        //detects keyword "Multiple" wrapped in square brackets which indicates a flagged enumeration/multiset later
        public static string multisetPattern = "(" + "[[]" + @"\s*" + "Multiple" + @"\s*" + "]" + ")";
        //values are indicated with 2 spaces an a name
        public static string parameterTypeValuePattern = "(" + descriptionTwoSpacePattern + "?" + spacePattern + "{2}" + namePattern + endOfLinePattern + ")";
        //parameter header
        public static string parameterTypePattern = "(" + descriptionOneSpacePattern + "?" + spacePattern + "Parameter" + @"\s+" + namePattern + @"\s*" + multisetPattern + "?" + codeBlockExpectedPattern
                                                         //values
                                                         + endOfLinePattern + "*" + parameterTypeValuePattern + "(" + parameterTypeValuePattern + "|" + endOfLinePattern + ")" + "*" + ")";
        //giving bounds for size of collections for a parameter of an thing
        public static string multiplicityPattern = "(" + "[[]" + "(" + numberWrappedWithSpacesPattern + "((<=)|<)" + ")?" + nameWrappedWithSpacesPattern + "(" + "((<=)|<)" + numberWrappedWithSpacesPattern + ")?" + "]" + ")";
        //accessing sub properties of an thing. thing names connected by a point
        public static string propertyPattern = "(" + namePattern + "(" + @"\s*" + "[.]" + @"\s*" + namePattern + ")*" + ")";
        //property that is wrapped by all whitespaces around it
        public static string propertyPatternWrappedWithSpacesPattern = "(" + @"\s*" + propertyPattern + @"\s*" + ")";
        
        /*//when benchmark is created for a sub property of an thing. Example Extract Area for a planar, non intersecting, closed Curve
        public static string castPattern = "(" + "[(]" + propertyPatternWrappedWithSpacesPattern + "[)]" + ")";*/
        
        /*//when benchmark does some calculations. CAUTION this is a very bad syntactical evaluator. Correct terms don't work with regex
        public static string mathematicalTermForPropertiesPattern = "(" + "{" + "[(]?" + "-?" + propertyPatternWrappedWithSpacesPattern +"[)]?" + "(" + "[(]?"+ arithmeticOperatorPattern + propertyPatternWrappedWithSpacesPattern + "[)]?" + ")*" + "}" + ")";
        */
        
        //No real checking here!!! Is still a whitespace and needs to implemented properly
        public static string mathematicalTermForPropertiesPattern = "(" + "{" + "[^}]*" + "}" + ")";

        //indicating the unit of the benchmark
        public static string unitPattern = "(" + "[[]" + nameWrappedWithSpacesPattern + "]" + ")";
        //benchmark starting with hash tag, optional cast, name of the benchmark and a unit
        public static string benchmarkPattern = "(" + "[#]" + @"\s*" + mathematicalTermForPropertiesPattern + "?" + @"\s*" + namePattern + "(" + @"\s*" + unitPattern + ")?" + ")";
        //optional description, name parameter type,  
        public static string parameterPattern = "(" + descriptionTwoSpacePattern + "?" + spacePattern + "{2}" + namePattern + "(" + "(" + @"\s*"
                                                          //multiplicity, name parameter, benchmark tag
                                                          + multiplicityPattern + @"\s*" + ")" + "|" + @"\s+" + ")" + namePattern + "(" + @"\s*" + benchmarkPattern + ")?" + endOfLinePattern + "?" + ")";
        //optional description, name, name parent things
        public static string thingPattern = "(" + descriptionOneSpacePattern + "?" + spacePattern + "Thing" + @"\s+" + namePattern + @"\s*" + argumentsPattern + "?" + codeBlockExpectedPattern
                                          //parameters
                                          + endOfLinePattern + "*" + parameterPattern + "(" + parameterPattern + "|" + endOfLinePattern + ")" + "*" + ")";

        public static string abstractionLevelHeaderPattern = "(" + descriptionPattern + "?" + "AbstractionLevel" + @"\s+" + namePattern + @"\s*" + argumentsPattern + "?" + codeBlockExpectedPattern + ")";

        //optional description , name , parent name abstractionLevel
        public static string abstractionLevelPattern = "(" + abstractionLevelHeaderPattern
                                          //all things and parameters
                                          + endOfLinePattern + "*" + "(" + thingPattern + "|" + parameterTypePattern + ")"
                                          + "(" + "(" + thingPattern + "|" + parameterTypePattern + "|" + endOfLinePattern + ")" + ")*" + ")";

        //All regex definition that are needed for parsing semIO code

        public static Regex ThingTypeRegex = new Regex(thingPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex ParameterRegex = new Regex(parameterPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);

        public static Regex ParameterTypeRegex = new Regex(parameterTypePattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex ParameterTypeValuesRegex = new Regex(parameterTypeValuePattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);

        public static Regex AbstractionLevelHeaderRegex = new Regex(abstractionLevelHeaderPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex AbstractionLevelRegex = new Regex(abstractionLevelPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);

        public static Regex DescriptionRegex = new Regex(descriptionPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex DescriptionOneSpaceRegex = new Regex(descriptionOneSpacePattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex DescriptionTwoSpaceRegex = new Regex(descriptionTwoSpacePattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);

        public static Regex NameRegex = new Regex(namePattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        
        public static Regex NumberRegex = new Regex(numberPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex CodeBlockExpectedRegex = new Regex(codeBlockExpectedPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex ArgumentRegex = new Regex(argumentPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex ArgumentsRegex = new Regex(argumentsPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex MultisetRegex = new Regex(multisetPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex MultiplicityRegex = new Regex(multiplicityPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex BenchmarkRegex = new Regex(benchmarkPattern, RegexOptions.Compiled | RegexOptions.ExplicitCapture);

        public static Regex WhiteSpaceRegex = new Regex(@"\s+", RegexOptions.Compiled | RegexOptions.ExplicitCapture);
        public static Regex SmallerOrEqualRegex = new Regex("<=");
    }
}
