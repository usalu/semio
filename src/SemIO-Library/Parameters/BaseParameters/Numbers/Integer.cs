namespace SemIOLibrary.Parameters.BaseParameters.Numbers
{
    public class Integer : Number
    {
        public new int Value { get; set; }

        public Integer()
        {
        }

        public Integer(int value)
        {
            Value = value;
        }
    }
}
