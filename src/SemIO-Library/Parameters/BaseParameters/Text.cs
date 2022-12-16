namespace SemIOLibrary.Parameters.BaseParameters
{
    public class Text : Parameter
    {
       public string Value { get; set; }

       public Text()
       {
       }

       public Text(string value)
       {
           Value = value;
       }
    }
}
