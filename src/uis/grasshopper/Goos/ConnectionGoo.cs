// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;

namespace Semio.UI.Grasshopper.Goos
{
    public class ConnectionGoo : SemioGoo<Connection>
    {
        public ConnectionGoo()
        {
            Value = new Connection();
        }
        public ConnectionGoo(Connection connection)
        {
            Value = connection;
        }
        public override IGH_Goo Duplicate() => new ConnectionGoo(Value.Clone());
        public override string TypeName => Connection.Descriptor.FullName;
        public override string TypeDescription => Connection.Descriptor.Declaration.LeadingComments;
    }
}