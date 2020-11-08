using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;
using SemIO.Parsing.ParserModels;
using SemIO.Parsing.ParserModels.Project.AbstractionLevels;
using SemIO.Parsing.ParserModels.Project.AbstractionLevels.Things;

namespace SemIO.Parsing
{
    /// <summary>
    /// This is the parser for semIO extracting all the semantic information.
    ///  NOTE: This can be rubbish as long as it follows the syntax.
    /// It is the compilers job to throw errors if the data doesn't make sense.
    /// </summary>
    public static class Parser
    {
        /// <summary>
        /// Main method for parsing all the relevant data from a semIO code string.
        /// It will add all the extracted abstraction levels.
        /// </summary>
        /// <param name="semIOCode">The semIO code that the user writes</param>
        /// <returns>Returns the parsed abstraction levels if the code IS inside the semIO language
        /// (meaning it contains at least one abstraction level) otherwise return null=</returns>
        public static List<AbstractionLevelModel> ParseAbstractionLevels(string semIOCode)
        {
            List<AbstractionLevelModel> abstractionLevels = new List<AbstractionLevelModel>();
            foreach (Match match in SemIORegexs.AbstractionLevelRegex.Matches(semIOCode))
            {
               
                //extracting name and parent name
                var parsedAbstractionLevelHeader = ParseAbstractionLevelHeader(match.Value);

                //extracting possible parameters
                var parsedParameterTypes = ParseParameterTypes(parsedAbstractionLevelHeader.RemainingCode);

                //extracting things
                var parsedThingTypes = Parser.ParseThingTypes(parsedParameterTypes.RemainingCode);

                abstractionLevels.Add(new AbstractionLevelModel(parsedAbstractionLevelHeader.Name,
                    parsedAbstractionLevelHeader.Description, 
                    parsedParameterTypes.ParameterTypes,
                    parsedThingTypes.ThingTypes,
                    parsedAbstractionLevelHeader.ParentName));
            }

            return abstractionLevels.Count == 0 ? null: abstractionLevels;
        }

        /// <summary>
        /// Parse all header information of the abstraction level (name, parent name, description)
        /// </summary>
        /// <param name="abstractionLevelCode">The code that describes the abstraction level INCLUDING optional description part</param>
        /// <returns>All the header information and the remaining code excluding the header code</returns>
        public static (string Name, string ParentName, string Description, string RemainingCode) ParseAbstractionLevelHeader(string abstractionLevelCode)
        {
            //extracting optional description
            var parsedAbstractionLevelDescription = ParseDescription(abstractionLevelCode);

            //The first name is the keyword "AbstractionLevel" and the second is the name of the abstraction level
            var headerMatch = SemIORegexs.AbstractionLevelHeaderRegex.Match(parsedAbstractionLevelDescription.RemainingCode);
            var nameMatches = SemIORegexs.NameRegex.Matches(headerMatch.Value);
            var parentName = "";

            //Checks if abstractionLevel is derived from another abstractionLevel.
            //If the there are 3 names the last one is the parent name
            if (nameMatches.Count == 3)
            {
                parentName = SemIORegexs.ArgumentRegex.Match(abstractionLevelCode).Value;
                //Remove brackets
                parentName = parentName.Substring(1, parentName.Length - 2);
            }

            return (nameMatches[1].Value, parentName, parsedAbstractionLevelDescription.Description,
                SemIORegexs.AbstractionLevelHeaderRegex.Replace(abstractionLevelCode,""));
        }

        /// <summary>
        /// Parse the optional description of a first order thing (only abstraction levels in the moment)
        /// and return the remaining code.
        /// </summary>
        /// <param name="abstractionLevelCode">The code that describes the abstraction level
        /// optionally (obviously) INCLUDING the description part</param>
        /// <returns>The pure thing (abstraction level) code
        /// and the optional description (empty string if none)</returns>
        public static (string Description, string RemainingCode) ParseDescription(string abstractionLevelCode)
        {
            var match = SemIORegexs.DescriptionRegex.Match(abstractionLevelCode);
            //remove quotation marks and return only text in between
            char[] seperator = new[] {'\"'};
            return match.Success
                ? (match.Value.Split(seperator, 3)[1], abstractionLevelCode.Substring(match.Value.Length))
                : ("", abstractionLevelCode);
        }

        /// <summary>
        /// Parse the optional description of a second order thing like a parameter type or a custom thing
        /// and returns the remaining code.
        /// </summary>
        /// <param name="secondOrderThingCode">A second order code means it starts with
        /// 1 semIO space unit (4 spaces or one tab)</param>
        /// <returns>The pure second order thing code and the optional description (empty string if none)</returns>
        public static (string Description, string RemainingCode) ParseDescriptionOneSpace(string secondOrderThingCode)
        {
            var match = SemIORegexs.DescriptionOneSpaceRegex.Match(secondOrderThingCode);
            //remove quotation marks and return only text in between
            char[] seperator = new[] { '\"' };
            return match.Success
                ? (match.Value.Split(seperator, 3)[1], secondOrderThingCode.Substring(match.Value.Length))
                : ("", secondOrderThingCode);
        }

        /// <summary>
        /// Parse the optional description of a third order thing like a parameter or a parameter type value
        /// and returns the remaining code.
        /// </summary>
        /// <param name="thirdOrderThingCode">A third order code means it starts with
        /// 2 semIO space units (4 spaces or one tab)</param>
        /// <returns>The pure second order thing code and the optional description (empty string if none)</returns>
        public static (string Description, string RemainingCode) ParseDescriptionTwoSpace(string thirdOrderThingCode)
        {
            var match = SemIORegexs.DescriptionTwoSpaceRegex.Match(thirdOrderThingCode);
            //remove quotation marks and return only text in between
            char[] seperator = new[] { '\"' };
            return match.Success
                ? (match.Value.Split(seperator, 3)[1], thirdOrderThingCode.Substring(match.Value.Length))
                : ("", thirdOrderThingCode);
        }

        /// <summary>
        /// Parse all the parameter types of one abstraction level.
        /// </summary>
        /// <param name="abstractionLevelCode">The code of an abstraction level containing the parameter types code</param>
        /// <returns>A list of all parameter types that belong to one abstraction level</returns>
        public static (List<ParameterType> ParameterTypes, string RemainingCode)
            ParseParameterTypes(string abstractionLevelCode) =>
        (SemIORegexs.ParameterTypeRegex.Matches(abstractionLevelCode).OfType<Match>()
                    .Select(x => ParseParameterType(x.Value)).ToList(),
                SemIORegexs.ParameterTypeRegex.Replace(abstractionLevelCode, ""));

        /// <summary>
        /// Parsing one parameter type for the parameter code part
        /// </summary>
        /// <param name="parameterCode">The code describing the parameter type INCLUDING optional description</param>
        /// <returns>The parsed parameter type of an abstraction level</returns>
        public static ParameterType ParseParameterType(string parameterCode)
        {
            var parsedDescription = ParseDescriptionOneSpace(parameterCode);
            //all parameter type values and their descriptions
            var parsedParameterTypeValues = 
                ParseParameterTypeValues(parsedDescription.RemainingCode);
            //first name is "Parameter", second the name of the custom parameter type,
            //third optionally "Multiset" and then all the value names
            var name = SemIORegexs.NameRegex.Matches(parsedParameterTypeValues.RemainingCode)[1].Value;
            //mainly important for enum type creating later
            var isMultiset = SemIORegexs.MultisetRegex.IsMatch(parsedDescription.RemainingCode);
            return new ParameterType(name, parsedDescription.Description,
                parsedParameterTypeValues.ParameterTypeValues.ToArray(), isMultiset,
                parsedParameterTypeValues.ParameterTypeValueDescription.ToArray());
        }

        /// <summary>
        /// Parse all the parameter type values and their corresponding descriptions. NOTE:
        /// If the parameter codes contains any thing besides the parameter code definition the parsing will fail.
        /// </summary>
        /// <param name="parameterCode">The code describing the parameter type WITHOUT general description.</param>
        /// <returns>All parameter type values, their corresponding description and the remaining code (the header code)</returns>
        public static (List<string>ParameterTypeValues, List<string> ParameterTypeValueDescription , string RemainingCode)
            ParseParameterTypeValues(string parameterCode)
        {
            List<string> parameterTypeValues = new List<string>();
            List<string> parameterTypeValueDescription = new List<string>();
            foreach (Match match in SemIORegexs.ParameterTypeValuesRegex.Matches(parameterCode))
            {
                //check for description and the remaining code can only be the name with whitespaces
                var parsedDescription = ParseDescriptionTwoSpace(match.Value);
                parameterTypeValueDescription.Add(parsedDescription.Description);
                parameterTypeValues.Add(SemIORegexs.NameRegex.Match(parsedDescription.RemainingCode).Value);
            }

            return (parameterTypeValues, parameterTypeValueDescription,
                parameterCode.Substring(0, SemIORegexs.ParameterTypeValuesRegex.Match(parameterCode).Index));
        }

        /// <summary>
        /// Parse all the things from an abstraction level code and return the code without the thing definitions.
        /// </summary>
        /// <param name="abstractionLevelCode">The code of an abstraction level containing all the thing definitions</param>
        /// <returns>All things and the remaining abstraction level code with the thing definitions removed</returns>
        public static (List<ThingModel> ThingTypes, string RemainingCode) ParseThingTypes(string abstractionLevelCode)
        {
            return (
                SemIORegexs.ThingTypeRegex.Matches(abstractionLevelCode).OfType<Match>().Select(x => ParseThingType(x.Value))
                    .ToList(),
                SemIORegexs.ThingTypeRegex.Replace(abstractionLevelCode, ""));
        }

        /// <summary>
        /// Parse an thing from the thing code. NOTE: If the code contains anything besides the code definition
        /// the parsing will not work.
        /// </summary>
        /// <param name="thingCode">The code describing th thing INCLUDING optional description</param>
        /// <returns>The thing that was parsed</returns>
        public static ThingModel ParseThingType(string thingCode)
        {
            //extract optional description
            var parsedDescription = ParseDescriptionOneSpace(thingCode);

            //extract name and parent names
            var parsedThingTypeName = ParseThingTypeName(parsedDescription.RemainingCode);

            return new ThingModel(parsedThingTypeName.Name, parsedDescription.Description,
                SemIORegexs.ParameterRegex.Matches(parsedThingTypeName.RemainingCode).OfType<Match>()
                    .Select(x => ParseParameter(x.Value)).ToList(), parsedThingTypeName.NameParents);
        }

        /// <summary>
        /// Extract the header information (thing name, the parent thing names)
        /// and return the remaining thing code without the header.
        /// NOTE: The parsing only works if the descriptions is NOT included.
        /// </summary>
        /// <param name="thingCode">Pure thing code WITHOUT description</param>
        /// <returns>Header information and remaining code with cut out header</returns>
        public static (string Name, List<string> NameParents, string RemainingCode) ParseThingTypeName(
            string thingCode)
        {
            var codeBlockExpectedMatch = SemIORegexs.CodeBlockExpectedRegex.Match(thingCode);
            //check if there are parent things defined that need to be inherited later
            var parentMatch = SemIORegexs.ArgumentsRegex.Match(thingCode.Substring(0, codeBlockExpectedMatch.Index));

            return (SemIORegexs.NameRegex.Matches(thingCode)[1].Value, parentMatch.Success
                    ? parentMatch.Value.Substring(1, parentMatch.Value.Length-2).Split(',').ToList()
                    : new List<string>(),
                parentMatch.Success
                    ? SemIORegexs.ArgumentsRegex.Replace(thingCode, "")
                    : thingCode.Substring(codeBlockExpectedMatch.Index + codeBlockExpectedMatch.Length));
        }

        /// <summary>
        /// Parse a single parameter of an thing including multiplicity and benchmark tag.
        /// </summary>
        /// <param name="parameterCode">Code describing the parameter with the parameter type,
        /// parameter name, multiplicity and benchmark tag</param>
        /// <returns>The parsed thing parameter</returns>
        public static ThingParameter ParseParameter(string parameterCode)
        {
            var parsedDescription = ParseDescriptionTwoSpace(parameterCode);
            var names = SemIORegexs.NameRegex.Matches(parsedDescription.RemainingCode);

            //If benchmark is found remove it before extracting the multiplicity
            //because both contain square brackets (type brackets)
            Match benchmarkMatch = SemIORegexs.BenchmarkRegex.Match(parameterCode);
            if (benchmarkMatch.Success)
                parameterCode = parameterCode.Substring(0,benchmarkMatch.Index);

            //If multiplicity is not provided the parameter has the fixed size of 1.
            var multiplicityMatch = SemIORegexs.MultiplicityRegex.Match(parameterCode);
            Multiplicity multiplicity = multiplicityMatch.Success
                ? ParseMultiplicity(multiplicityMatch.Value)
                : new Multiplicity(1);

            //extract optional benchmark tag
            var benchmark = ParseBenchmark(parsedDescription.RemainingCode);

            return new ThingParameter(names[multiplicity.ExactSize == 1 ? 1 : 2].Value,
                parsedDescription.Description, names[0].Value, multiplicity);
        }

        /// <summary>
        /// Extract the multiplicity from the multiplicity tag of a thing parameter
        /// </summary>
        /// <param name="codeMultiplicity">The code of the multiplicity tag</param>
        /// <returns>Extracted multiplicity from the multiplicity tag</returns>
        public static Multiplicity ParseMultiplicity(string codeMultiplicity)
        {
            //variable name
            var nameMatch = SemIORegexs.NameRegex.Match(codeMultiplicity);
            //amount of bounds; Either both side bounded or only lower or only upper bounded
            var numberMatches = SemIORegexs.NumberRegex.Matches(codeMultiplicity);
            //the amount of smallerOrEqual tokens gives the amount of smaller tokens
            var smallerOrEqualMatches = SemIORegexs.SmallerOrEqualRegex.Matches(codeMultiplicity);

            uint? lowerBoundary = null;
            uint? upperBoundary = null;

            switch (numberMatches.Count)
            {
                //Two numbers means two bounds
                case 2:
                {
                    switch (smallerOrEqualMatches.Count)
                    {
                        //both boundaries as written
                        case 2:
                            lowerBoundary = uint.Parse(numberMatches[0].Value);
                            upperBoundary = uint.Parse(numberMatches[1].Value);
                            break;
                        //one boundary must be adjusted because of one smallerThan needs to be put to smallerOrEqual
                        case 1:
                            //if the first token is smallerOrEqual it must be before the variable name
                            var smaller = (uint) (smallerOrEqualMatches[0].Index < nameMatch.Index ? 0 : 1);
                            lowerBoundary = uint.Parse(numberMatches[0].Value) + smaller;
                            upperBoundary = uint.Parse(numberMatches[1].Value) - 1 + smaller;
                            break;
                        //both smallerThan must be converted to smallerOrEqual
                        default:
                            lowerBoundary = uint.Parse(numberMatches[0].Value) + 1;
                            upperBoundary = uint.Parse(numberMatches[1].Value) - 1;
                            break;
                    }

                    break;
                }
                //only one boundary given
                case 1:
                {
                    //smaller is there for conversion of smallerThan to smallerOrEqual
                    var smaller = (uint) (smallerOrEqualMatches.Count == 1 ? 0 : 1);
                    //if number starts before the name it is the lower boundary
                    if (numberMatches[0].Index < nameMatch.Index)
                        lowerBoundary = uint.Parse(numberMatches[0].Value) + smaller;
                    else
                        upperBoundary = uint.Parse(numberMatches[1].Value) - smaller;
                    break;
                }
            }

            return new Multiplicity(nameMatch.Value, lowerBoundary, upperBoundary);
        }

        /// <summary>
        /// NOT YET IMPLEMENTED. Placeholder only!
        /// </summary>
        /// <param name="benchMarkCode"></param>
        /// <returns></returns>
        public static Benchmark ParseBenchmark(string benchMarkCode)
        {
            return new Benchmark();
        }
    }
}