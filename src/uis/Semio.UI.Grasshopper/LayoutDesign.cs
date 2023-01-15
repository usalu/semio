using Grasshopper;
using Grasshopper.Kernel;
using Rhino.Geometry;
using System;
using System.Collections.Generic;
using Google.Protobuf.Collections;
using Semio;
using Grpc.Net.Client;
using Semio.Gateway.V1;
using Semio.Model.V1;
using System.Net.Http;
using Grpc.Core;

namespace Semio.UI.Grasshopper
{
    public class LayoutDesign : GH_Component
    {
        /// <summary>
        /// Each implementation of GH_Component must provide a public 
        /// constructor without any arguments.
        /// Category represents the Tab in which the component will appear, 
        /// Subcategory the panel. If you use non-existing tab or panel names, 
        /// new tabs/panels will automatically be created.
        /// </summary>
        public LayoutDesign()
          : base("LayoutDesign", "LayoutDesign",
            "Description",
            "Semio", "Design")
        {
        }

        /// <summary>
        /// Registers all the input parameters for this component.
        /// </summary>
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddBooleanParameter("Run", "R", "", GH_ParamAccess.item);
        }

        /// <summary>
        /// Registers all the output parameters for this component.
        /// </summary>
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddGenericParameter("Design", "Design", "", GH_ParamAccess.item);
        }

        /// <summary>
        /// This is the method that actually does the work.
        /// </summary>
        /// <param name="DA">The DA object can be used to retrieve data from input parameters and 
        /// to store data in output parameters.</param>
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            // Http handler works on .Net Framework only with TLS
            // https://learn.microsoft.com/en-us/aspnet/core/grpc/netstandard?view=aspnetcore-7.0
            //AppContext.SetSwitch("System.Net.Http.SocketsHttpHandler.Http2UnencryptedSupport", true);
            var channel = GrpcChannel.ForAddress("http://localhost:50001", new GrpcChannelOptions
            {
                HttpHandler = new WinHttpHandler(),
                Credentials = ChannelCredentials.Insecure
            });

            var client = new Gateway.V1.GatewayService.GatewayServiceClient(channel);
            var request = new LayoutDesignRequest()
            {
                Layout = new Layout()
                {
                    Sobjects =
                    {
                        new Sobject()
                        {
                            Id = "1"
                        }
                    }
                }
            };

            var response = client.LayoutDesign(request);

            DA.SetData(0, response);
        }

        /// <summary>
        /// Provides an Icon for every component that will be visible in the User Interface.
        /// Icons need to be 24x24 pixels.
        /// You can add image files to your project resources and access them like this:
        /// return Resources.IconForThisComponent;
        /// </summary>
        protected override System.Drawing.Bitmap Icon => null;

        /// <summary>
        /// Each component must have a unique Guid to identify it. 
        /// It is vital this Guid doesn't change otherwise old ghx files 
        /// that use the old ID will partially fail during loading.
        /// </summary>
        public override Guid ComponentGuid => new Guid("15ad0008-1e40-41ff-8e38-116102d7488a");
    }
}