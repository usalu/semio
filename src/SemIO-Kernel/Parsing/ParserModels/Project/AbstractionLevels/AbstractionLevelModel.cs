using System;
using System.Collections.Generic;
using SemIO.Parsing.ParserModels.Project.AbstractionLevels.Things;

namespace SemIO.Parsing.ParserModels.Project.AbstractionLevels
{
    /// <summary>
    /// Main class of a semIO project. This class brings all the functionality from semIO together
    ///  and is the main interface in this framework.
    /// </summary>
    public class AbstractionLevelModel : InformedClass
    {
        public string ParentAbstractionLevelName { get; }
        public List<ParameterType> ParameterTypes { get; }
        public List<ThingModel> ThingTypes { get; }

        public AbstractionLevelModel(string name, string description, string parentAbstractionLevelName = "") : base(name, description)
        {
            ParentAbstractionLevelName = parentAbstractionLevelName;
            ParameterTypes = new List<ParameterType>();
            ThingTypes = new List<ThingModel>();
        }

        public AbstractionLevelModel(string name, string description, List<ParameterType> customParameterTypes,
            List<ThingModel> thingTypes, string parentAbstractionLevelName = "") 
            : base(name, description)
        {
            ParameterTypes = customParameterTypes;
            ThingTypes = thingTypes;
            ParentAbstractionLevelName = parentAbstractionLevelName;
        }

        public ParameterType AddParameterType(string name, string description, string[] values, bool isMultiset)
        {
            if(ExistsParameterTypeName(name))
                throw new Exception("This parameter type already exists in this level of abstraction");

            ParameterType customParameterTypeName = new ParameterType(name, description, values, isMultiset);
            ParameterTypes.Add(customParameterTypeName);
            return customParameterTypeName;
        }

        /// <summary>
        /// Add a parameter type to the abstraction level if it doesn't exist yet inside this abstraction level.
        /// Otherwise error will be thrown.
        /// </summary>
        /// <param name="parameterType">Parameter type to add to abstraction level</param>
        /// <returns>The added parameter type or error if it already exists inside this abstraction level</returns>
        public ParameterType AddParameterType(ParameterType parameterType)
        {
            if (ExistsParameterTypeName(parameterType.Name))
                throw new Exception("This parameter type already exists in this level of abstraction");

            ParameterTypes.Add(parameterType);
            return parameterType;
        }

        public bool ExistsThingType(string name) => ThingTypes.Exists(x => x.Name == name);
        public bool ExistsParameterTypeName(string name) => ParameterTypes.Exists(x => x.Name == name);

    }
}
