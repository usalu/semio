using System;

namespace SemIO.Parsing.ParserModels.Project.AbstractionLevels.Things
{
    /// <summary>
    /// Limits the size of a collection. Either way only a specific number is allowed, one boundary is set,
    /// both boundaries are set or every size is allowed. This information has an impact on the compilation of
    /// collection of parameters.
    /// </summary>
    public class Multiplicity
    {
        //open interval lower boundary
        private uint? _lowerBoundary;
        //open interval upper boundary
        private uint? _upperBoundary;
        public string MultiplicityVariable { get; set; }
        public uint? ExactSize { get; set; }

        public uint? LowerBoundary
        {
            get => _lowerBoundary;
            set
            {
                if (value != null && _upperBoundary != null)
                    if (value >= _upperBoundary)
                        throw new Exception("Lower boundary must be smaller than upper boundary.");
                _lowerBoundary = value;
            }
        }

        public uint? UpperBoundary
        {
            get => _upperBoundary;
            set
            {
                if(value != null && _lowerBoundary != null)
                    if(value <= _lowerBoundary)
                        throw new Exception("Upper boundary must be larger than lower boundary.");
                _upperBoundary = value;
            }
        }
        public bool IsLowerBounded => _lowerBoundary != null;
        public bool IsUpperBounded => _upperBoundary != null;

        public bool IsBounded => IsLowerBounded && IsUpperBounded;
        public bool IsExactSize => ExactSize != null;

        public void SetSize(uint size)
        {
            MultiplicityVariable = null;
            LowerBoundary = null;
            UpperBoundary = null;
            ExactSize = size;
        }

        public void SetInterval(string multiplicityVariable, uint? lowerBoundary = null, uint? upperBoundary = null)
        {
            ExactSize = null;
            LowerBoundary = lowerBoundary;
            UpperBoundary = upperBoundary;
            MultiplicityVariable = multiplicityVariable;
        }

        public Multiplicity(string multiplicityVariable,uint? lowerBoundary = null, uint? upperBoundary = null)
        { 
            SetInterval(multiplicityVariable,lowerBoundary,upperBoundary);
        }

        public Multiplicity(uint size)
        {
            SetSize(size);
        }
    }
}
